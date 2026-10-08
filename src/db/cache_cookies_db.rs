// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/cache_cookies_db.rs
// Description: Dedicated SQLite database for Cookie Policies & Cache Management.
// ============================================================================

use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Cookie handling and storage policy for an origin domain.
///
/// I have written this struct to allow granular per-domain storage controls:
/// - `block_third_party`: Disallows third-party tracker cookies from setting state.
/// - `clear_on_exit`: Wipes local cookies and storage as soon as the active tab closes.
/// - `isolated_jar`: Partitions storage into an isolated jar to defeat cross-site tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookiePolicy {
    pub domain: String,
    pub block_third_party: bool,
    pub clear_on_exit: bool,
    pub isolated_jar: bool,
    pub last_modified: String,
}

/// Record of cache or cookie cleanup execution.
///
/// I have written this audit struct to verify that user-initiated or automatic
/// purges actually executed, logging the cleanup category, byte count, and timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupAudit {
    pub id: i64,
    pub cleanup_type: String,
    pub details: String,
    pub bytes_cleared: i64,
    pub timestamp: String,
}

/// Dedicated Cache & Cookies Database Manager.
///
/// I have upgraded this dedicated database manager to use fine-tuned SQLite PRAGMAs
/// including WAL mode, 64MB cache size, and busy timeout handlers. I wrote this
/// architecture because cookies, Web Storage, and disk cache represent the physical
/// persistence attack surface of tracking. By maintaining storage rules and cleanup
/// records in `cache_cookies.db`, NORVA manages storage lifecycles without touching history.
#[derive(Clone)]
pub struct CacheCookiesDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl CacheCookiesDatabase {
    /// Opens or initializes the cache & cookies database.
    ///
    /// I have written this initialization routine to enforce WAL mode and initialize
    /// tables for per-site storage rules, cleanup verification logs, and storage settings.
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        // High-performance SQLite tuning: WAL mode, memory temp store, 64MB cache
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -64000;
             PRAGMA busy_timeout = 5000;"
        )?;

        // Per-domain cookie and storage rules
        conn.execute(
            "CREATE TABLE IF NOT EXISTS domain_cookie_policies (
                domain TEXT PRIMARY KEY,
                block_third_party BOOLEAN NOT NULL DEFAULT 1,
                clear_on_exit BOOLEAN NOT NULL DEFAULT 1,
                isolated_jar BOOLEAN NOT NULL DEFAULT 0,
                last_modified TEXT NOT NULL
            );",
            [],
        )?;

        // Audit log of cache and cookie flushes
        conn.execute(
            "CREATE TABLE IF NOT EXISTS storage_cleanups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                cleanup_type TEXT NOT NULL,
                details TEXT NOT NULL,
                bytes_cleared INTEGER NOT NULL DEFAULT 0,
                timestamp TEXT NOT NULL
            );",
            [],
        )?;

        // Storage configuration switches
        conn.execute(
            "CREATE TABLE IF NOT EXISTS storage_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
            [],
        )?;

        let defaults = [
            ("block_all_third_party_cookies", "true"),
            ("auto_clear_cache_on_exit", "true"),
            ("max_cache_size_mb", "256"),
            ("ephemeral_session_storage", "true"),
        ];

        for (k, v) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO storage_settings (key, value) VALUES (?, ?)",
                params![k, v],
            )?;
        }

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Retrieves or defaults cookie policy for an origin domain.
    ///
    /// I have written this lookup function so NORVA can dynamically enforce
    /// whether a domain is permitted to write persistent cookies or must be wiped on tab exit.
    pub fn get_cookie_policy(&self, domain: &str) -> CookiePolicy {
        let conn = self.conn.lock().unwrap();
        let query_res = conn.query_row(
            "SELECT domain, block_third_party, clear_on_exit, isolated_jar, last_modified
             FROM domain_cookie_policies
             WHERE domain = ?",
            [domain],
            |row| {
                Ok(CookiePolicy {
                    domain: row.get(0)?,
                    block_third_party: row.get::<_, i64>(1)? == 1,
                    clear_on_exit: row.get::<_, i64>(2)? == 1,
                    isolated_jar: row.get::<_, i64>(3)? == 1,
                    last_modified: row.get(4)?,
                })
            },
        );

        query_res.unwrap_or_else(|_| CookiePolicy {
            domain: domain.to_string(),
            block_third_party: true,
            clear_on_exit: true,
            isolated_jar: false,
            last_modified: Utc::now().to_rfc3339(),
        })
    }

    /// Sets or updates an origin domain cookie policy.
    pub fn set_cookie_policy(
        &self,
        domain: &str,
        block_third_party: bool,
        clear_on_exit: bool,
        isolated_jar: bool,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO domain_cookie_policies (domain, block_third_party, clear_on_exit, isolated_jar, last_modified)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(domain) DO UPDATE SET
                block_third_party = excluded.block_third_party,
                clear_on_exit = excluded.clear_on_exit,
                isolated_jar = excluded.isolated_jar,
                last_modified = excluded.last_modified",
            params![domain, block_third_party as i64, clear_on_exit as i64, isolated_jar as i64, now],
        )?;

        Ok(())
    }

    /// Logs an audited storage purge event with freed byte counts.
    pub fn log_cleanup(&self, cleanup_type: &str, details: &str, bytes_cleared: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO storage_cleanups (cleanup_type, details, bytes_cleared, timestamp)
             VALUES (?, ?, ?, ?)",
            params![cleanup_type, details, bytes_cleared, now],
        )?;

        Ok(())
    }

    /// Retrieves recent cleanup audit logs.
    pub fn get_recent_cleanups(&self, limit: usize) -> Result<Vec<CleanupAudit>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, cleanup_type, details, bytes_cleared, timestamp
             FROM storage_cleanups
             ORDER BY id DESC
             LIMIT ?"
        )?;

        let rows = stmt.query_map([limit as i64], |row| {
            Ok(CleanupAudit {
                id: row.get(0)?,
                cleanup_type: row.get(1)?,
                details: row.get(2)?,
                bytes_cleared: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;

        let mut audits = Vec::new();
        for r in rows {
            audits.push(r?);
        }

        Ok(audits)
    }

    /// Deletes all historical cleanup audit records.
    pub fn clear_cleanups(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM storage_cleanups;", [])?;
        Ok(())
    }
}
