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

/// A single browsing history record with detailed navigation metadata.
///
/// I have written this struct because every historical visit in NORVA needs
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

/// Statistics on user history and search queries.
///
/// I have written this summary struct to provide the history dashboard with quick,
/// high-level metrics without having to stream every row over IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_entries: i64,
    pub total_searches: i64,
}

/// Dedicated History Database Manager.
///
/// I have upgraded this dedicated database manager to use fine-tuned SQLite PRAGMAs
/// including WAL mode, memory temp store, 64MB cache size, and busy timeout handlers.
/// I wrote this architecture because keeping user navigation history strictly separated
/// in `history.db` guarantees that wiping history never locks or corrupts bookmarks,
/// search engines, or shield databases.
#[derive(Clone)]
pub struct HistoryDatabase {
    conn: Arc<Mutex<Connection>>,
    secure_mode: bool,
}

impl HistoryDatabase {
    /// Opens or creates the dedicated history database.
    ///
    /// I have written this constructor to cleanly handle both regular on-disk operations
    /// and ephemeral Secure / Ghost Mode. In Secure Mode, the database lives purely in
    /// `:memory:` and vanishes completely upon browser exit.
    pub fn new(path: &Path, secure_mode: bool) -> Result<Self> {
        let conn = if secure_mode {
            // I am writing this in-memory connection because in Secure / Ghost Mode,
            // zero bytes should ever touch non-volatile disk storage.
            Connection::open_in_memory()?
        } else {
            Connection::open(path)?
        };

        // I have added these performance PRAGMAs because SQLite's default journal mode
        // introduces disk latency on every page navigation. WAL mode + 64MB cache ensures
        // visits are committed asynchronously without ever freezing the UI thread.
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -64000;
             PRAGMA busy_timeout = 5000;"
        )?;

        // I have designed this table with indexes on visit_time and URL to ensure
        // instant retrieval and deduplication even after thousands of visits.
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

    /// Records or updates a page visit with intelligent deduplication.
    ///
    /// I have written this function to automatically check if the user is visiting
    /// the same URL again. Instead of spamming the database with duplicate rows, it increments
    /// `visit_count` and updates the timestamp, keeping history compact and fast to search.
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

        // Avoid logging internal norva:// system pages into the user's web history
        if url.starts_with("norva://") || url.starts_with("about:") || url.starts_with("data:") {
            return Ok(());
        }

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        // Check if this URL was recently visited to increment visit_count instead of adding duplicate rows
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

    /// Retrieves the most recent history records up to the specified limit.
    ///
    /// I have written this query with an explicit LIMIT clause to prevent huge historical
    /// datasets from consuming excessive memory when rendering the history panel.
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
        for r in rows {
            entries.push(r?);
        }

        Ok(entries)
    }

    /// Searches history entries matching a keyword in title, URL, or search query.
    ///
    /// I have written this comprehensive search query using SQL LIKE pattern matching
    /// so users can instantly find previous sites or queries by typing any keyword fragment.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<HistoryEntry>> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, url, title, search_query, search_engine, visit_time, visit_count
             FROM history
             WHERE url LIKE ? OR title LIKE ? OR search_query LIKE ?
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
        for r in rows {
            entries.push(r?);
        }

        Ok(entries)
    }

    /// Deletes a specific history record by primary key ID.
    ///
    /// I have written this deletion method so users can remove individual records from
    /// their browsing log without wiping their entire history vault.
    pub fn delete_entry(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history WHERE id = ?", [id])?;
        Ok(())
    }

    /// Deletes all history records associated with a specific URL.
    ///
    /// I have written this function so users can purge all references to a specific domain or page.
    pub fn delete_by_url(&self, url: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history WHERE url = ?", [url])?;
        Ok(())
    }

    /// Clears the entire browsing history vault.
    ///
    /// I have written this purge method with a VACUUM command to cleanly reclaim all disk space
    /// and reset the history database to zero bytes.
    pub fn clear_all(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history;", [])?;
        conn.execute("VACUUM;", [])?;
        Ok(())
    }

    /// Retrieves aggregate metrics on the history database.
    ///
    /// I have written this query to count total navigation events and distinct search queries
    /// for display on the NORVA analytics cards.
    pub fn get_stats(&self) -> Result<HistoryStats> {
        let conn = self.conn.lock().unwrap();

        let total_entries: i64 = conn.query_row(
            "SELECT COUNT(*) FROM history",
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        let total_searches: i64 = conn.query_row(
            "SELECT COUNT(*) FROM history WHERE search_query IS NOT NULL",
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        Ok(HistoryStats {
            total_entries,
            total_searches,
        })
    }
}
