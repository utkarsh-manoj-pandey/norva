// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/downloads_db.rs
// Description: Dedicated SQLite database for downloaded files and transfer logs.
// ============================================================================

use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A single download record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEntry {
    pub id: i64,
    pub url: String,
    pub filename: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub status: String,
    pub created_at: String,
}

/// Dedicated Downloads Database Manager.
#[derive(Clone)]
pub struct DownloadsDatabase {
    conn: Arc<Mutex<Connection>>,
    secure_mode: bool,
}

impl DownloadsDatabase {
    /// Opens or creates the dedicated downloads database.
    pub fn new(path: &Path, secure_mode: bool) -> Result<Self> {
        let conn = if secure_mode {
            Connection::open_in_memory()?
        } else {
            Connection::open(path)?
        };

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -16000;
             PRAGMA busy_timeout = 5000;",
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS downloads (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                filename TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size_bytes INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'completed',
                created_at TEXT NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_downloads_time ON downloads(created_at DESC);",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            secure_mode,
        })
    }

    /// Records a new download in the database.
    pub fn record_download(
        &self,
        url: &str,
        filename: &str,
        file_path: &str,
        size_bytes: i64,
        status: &str,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO downloads (url, filename, file_path, file_size_bytes, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![url, filename, file_path, size_bytes, status, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Updates status of an existing download.
    pub fn update_status(&self, id: i64, status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE downloads SET status = ?1 WHERE id = ?2;",
            params![status, id],
        )?;
        Ok(())
    }

    /// Retrieves recent downloads up to limit.
    pub fn get_recent(&self, limit: usize) -> Result<Vec<DownloadEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, url, filename, file_path, file_size_bytes, status, created_at
             FROM downloads
             ORDER BY id DESC
             LIMIT ?1;",
        )?;

        let rows = stmt.query_map([limit as i64], |row| {
            Ok(DownloadEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                filename: row.get(2)?,
                file_path: row.get(3)?,
                file_size_bytes: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }

    /// Deletes a download record by ID.
    pub fn delete_download(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM downloads WHERE id = ?1;", params![id])?;
        Ok(())
    }

    /// Purges all download records.
    pub fn clear_all(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM downloads;", [])?;
        Ok(())
    }
}
