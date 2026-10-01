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

/// Cookie handling and storage policy for a domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookiePolicy {
    pub domain: String,
    pub block_third_party: bool,
    pub clear_on_exit: bool,
    pub isolated_jar: bool,
    pub last_modified: String,
}

/// Record of cache or cookie cleanup execution.
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
/// I am writing this dedicated database manager because cookies and cache memory
/// represent the physical persistence layer of tracking. By keeping storage policies,
/// domain cookie rules, and cleanup verification logs in `cache_cookies.db`,
/// the browser can manage site storage lifecycle without ever touching user history or settings.
#[derive(Clone)]
pub struct CacheCookiesDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl CacheCookiesDatabase {
    /// Opens or initializes the cache & cookies database.
    ///
    /// I have written this initialization routine to enforce WAL mode and initialize
    /// tables for per-site storage rules and audit logs.
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
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

    /// Retrieves or defaults cookie policy for a domain.
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

    /// Sets or updates a domain cookie policy.
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
            "INSERT INTO domain_cookie_policies 
             (domain, block_third_party, clear_on_exit, isolated_jar, last_modified)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(domain) DO UPDATE SET
                block_third_party = excluded.block_third_party,
                clear_on_exit = excluded.clear_on_exit,
                isolated_jar = excluded.isolated_jar,
                last_modified = excluded.last_modified",
            params![domain, block_third_party as i64, clear_on_exit as i64, isolated_jar as i64, now],
        )?;

        Ok(())
    }

    /// Records a cleanup action in the audit log.
    pub fn log_cleanup(&self, cleanup_type: &str, details: &str, bytes: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO storage_cleanups (cleanup_type, details, bytes_cleared, timestamp)
             VALUES (?, ?, ?, ?)",
            params![cleanup_type, details, bytes, now],
        )?;

        Ok(())
    }

    /// Retrieves all cleanup audit logs.
    pub fn get_cleanup_audits(&self, limit: usize) -> Result<Vec<CleanupAudit>> {
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

        let mut list = Vec::new();
        for item in rows {
            if let Ok(c) = item {
                list.push(c);
            }
        }
        Ok(list)
    }

    /// Retrieves all configured cookie policies.
    pub fn get_all_policies(&self) -> Result<Vec<CookiePolicy>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT domain, block_third_party, clear_on_exit, isolated_jar, last_modified
             FROM domain_cookie_policies
             ORDER BY domain ASC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(CookiePolicy {
                domain: row.get(0)?,
                block_third_party: row.get::<_, i64>(1)? == 1,
                clear_on_exit: row.get::<_, i64>(2)? == 1,
                isolated_jar: row.get::<_, i64>(3)? == 1,
                last_modified: row.get(4)?,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            if let Ok(p) = item {
                list.push(p);
            }
        }
        Ok(list)
    }
}
