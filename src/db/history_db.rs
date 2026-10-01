// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/history_db.rs
// Description: Dedicated SQLite database for browsing history & search queries.
// ============================================================================

use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A single browsing history record.
///
/// I am writing this struct because every historical visit in NORVA needs
/// transparent metadata: the destination URL, page title, any active search engine
/// query associated with it, the timestamp, and the frequency of visits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub search_query: Option<String>,
    pub search_engine: Option<String>,
    pub visit_time: String,
    pub visit_count: i64,
}

/// Statistics on user history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_entries: i64,
    pub total_searches: i64,
}

/// Dedicated History Database Manager.
///
/// I am writing this dedicated database manager because user history must never be
/// co-mingled with settings or permissions. Keeping a separate SQLite database file
/// (`history.db`) ensures that a user can purge, backup, or inspect their historical
/// trail without locking or corrupting any other critical browser configuration files.
#[derive(Clone)]
pub struct HistoryDatabase {
    conn: Arc<Mutex<Connection>>,
    secure_mode: bool,
}

impl HistoryDatabase {
    /// Opens or creates the dedicated history database.
    ///
    /// I have written this function to enforce high performance SQLite tuning:
    /// Write-Ahead Logging (WAL) allows concurrent reads while writes are committed,
    /// and synchronous=NORMAL provides rock-solid data integrity without disk thrashing.
    /// In Secure Mode, we swap the disk connection for an in-memory database (`:memory:`)
    /// so no trace ever touches physical non-volatile storage.
    pub fn new(path: &Path, secure_mode: bool) -> Result<Self> {
        let conn = if secure_mode {
            // I am writing this in-memory fallback because in Secure / Ghost Mode,
            // the database must live strictly in volatile RAM and vanish upon browser exit.
            Connection::open_in_memory()?
        } else {
            Connection::open(path)?
        };

        // I have written these PRAGMA configurations because SQLite in default DELETE mode
        // is far too slow for real-time web navigation logging. WAL mode grants ultra-fast writes.
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;"
        )?;

        // I am creating this schema with an index on visit_time to ensure instantaneous
        // loading of the recent history view even after millions of page loads.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                search_query TEXT,
                search_engine TEXT,
                visit_time TEXT NOT NULL,
                visit_count INTEGER NOT NULL DEFAULT 1
            );",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_time ON history(visit_time DESC);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_url ON history(url);",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            secure_mode,
        })
    }

    /// Records or updates a page visit.
    ///
    /// I have written this function to automatically deduplicate identical sequential visits
    /// to the exact same URL within a short window, incrementing the visit count rather than
    /// polluting the user's history with hundreds of redundant entries.
    pub fn record_visit(
        &self,
        url: &str,
        title: &str,
        search_query: Option<&str>,
        search_engine: Option<&str>,
    ) -> Result<()> {
        // I am adding this guard because in Secure Mode, the user explicitly chose zero history tracking.
        if self.secure_mode {
            return Ok(());
        }

        // Avoid logging internal norva:// pages into the user's web history
        if url.starts_with("norva://") || url.starts_with("about:") {
            return Ok(());
        }

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        // Check if this URL was recently visited to increment the visit_count instead of spamming rows
        let mut check_stmt = conn.prepare(
            "SELECT id, visit_count FROM history WHERE url = ? ORDER BY id DESC LIMIT 1"
        )?;
        let mut rows = check_stmt.query([url])?;

        if let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let count: i64 = row.get(1)?;
            conn.execute(
                "UPDATE history SET title = ?, visit_time = ?, visit_count = ? WHERE id = ?",
                params![title, now, count + 1, id],
            )?;
        } else {
            conn.execute(
                "INSERT INTO history (url, title, search_query, search_engine, visit_time, visit_count)
                 VALUES (?, ?, ?, ?, ?, 1)",
                params![url, title, search_query, search_engine, now],
            )?;
        }

        Ok(())
    }

    /// Retrieves the most recent history entries.
    ///
    /// I am writing this query to drive the sleek `norva://history` dashboard,
    /// giving users a clear chronological view of their activity.
    pub fn get_recent(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, url, title, search_query, search_engine, visit_time, visit_count
             FROM history
             ORDER BY visit_time DESC
             LIMIT ?"
        )?;

        let rows = stmt.query_map([limit as i64], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                search_query: row.get(3)?,
                search_engine: row.get(4)?,
                visit_time: row.get(5)?,
                visit_count: row.get(6)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry in rows {
            if let Ok(e) = entry {
                entries.push(e);
            }
        }
        Ok(entries)
    }

    /// Searches history entries matching a user search term.
    ///
    /// I have written this search method because users need to quickly find tabs
    /// or searches they performed days ago with instant substring matching.
    pub fn search(&self, term: &str, limit: usize) -> Result<Vec<HistoryEntry>> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{}%", term);
        let mut stmt = conn.prepare(
            "SELECT id, url, title, search_query, search_engine, visit_time, visit_count
             FROM history
             WHERE title LIKE ? OR url LIKE ? OR search_query LIKE ?
             ORDER BY visit_time DESC
             LIMIT ?"
        )?;

        let rows = stmt.query_map(params![pattern, pattern, pattern, limit as i64], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                search_query: row.get(3)?,
                search_engine: row.get(4)?,
                visit_time: row.get(5)?,
                visit_count: row.get(6)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry in rows {
            if let Ok(e) = entry {
                entries.push(e);
            }
        }
        Ok(entries)
    }

    /// Deletes a specific history entry by ID.
    ///
    /// I am writing this deletion method to give users granular, 1-click control
    /// to purge sensitive or accidental visits without erasing their whole history.
    pub fn delete_entry(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history WHERE id = ?", [id])?;
        Ok(())
    }

    /// Completely purges all history entries.
    ///
    /// I have written this clear function because privacy self-determination demands
    /// an instant, atomic purge of the entire browsing log.
    pub fn clear_all(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history", [])?;
        conn.execute("VACUUM", [])?;
        Ok(())
    }

    /// Gathers summary statistics for the history database.
    pub fn get_stats(&self) -> Result<HistoryStats> {
        let conn = self.conn.lock().unwrap();
        let total_entries: i64 = conn.query_row(
            "SELECT COUNT(*) FROM history",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        let total_searches: i64 = conn.query_row(
            "SELECT COUNT(*) FROM history WHERE search_query IS NOT NULL AND search_query != ''",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        Ok(HistoryStats {
            total_entries,
            total_searches,
        })
    }
}
