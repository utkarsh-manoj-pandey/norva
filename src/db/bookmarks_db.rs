// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/bookmarks_db.rs
// Description: Dedicated SQLite database for user bookmarks and quick speed dials.
// ============================================================================

use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A single bookmark record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub icon: String,
    pub folder: String,
    pub created_at: String,
    pub visit_count: i64,
}

/// Dedicated Bookmarks Database Manager.
#[derive(Clone)]
pub struct BookmarksDatabase {
    conn: Arc<Mutex<Connection>>,
    secure_mode: bool,
}

impl BookmarksDatabase {
    /// Opens or creates the dedicated bookmarks database.
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
             PRAGMA cache_size = -32000;
             PRAGMA busy_timeout = 5000;",
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                icon TEXT NOT NULL DEFAULT '🌐',
                folder TEXT NOT NULL DEFAULT 'Bookmarks',
                created_at TEXT NOT NULL,
                visit_count INTEGER NOT NULL DEFAULT 0
            );",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_url ON bookmarks(url);",
            [],
        )?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            secure_mode,
        };

        db.seed_defaults_if_empty()?;
        Ok(db)
    }

    /// Seeds default privacy and tech bookmarks if the table is empty.
    fn seed_defaults_if_empty(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM bookmarks", [], |row| row.get(0))?;
        if count == 0 {
            let defaults = [
                ("https://duckduckgo.com", "DuckDuckGo", "🦆", "Search"),
                ("https://ahmia.fi", "Ahmia Tor", "🧅", "Privacy"),
                ("https://www.eff.org", "EFF Privacy", "🛡️", "Privacy"),
                ("https://www.torproject.org", "Tor Project", "🧅", "Privacy"),
                ("https://www.wikipedia.org", "Wikipedia", "📚", "Reference"),
                ("https://www.rust-lang.org", "Rust Lang", "🦀", "Development"),
                ("https://github.com", "GitHub", "🐙", "Development"),
                ("norva://shields", "Shields Center", "🛡️", "Norva"),
                ("norva://history", "History Vault", "📜", "Norva"),
                ("norva://bookmarks", "Bookmarks", "★", "Norva"),
                ("norva://downloads", "Downloads", "📥", "Norva"),
                ("norva://passwords", "Passwords", "🔑", "Norva"),
                ("norva://settings", "Settings", "⚙️", "Norva"),
            ];

            let now = Utc::now().to_rfc3339();
            for (url, title, icon, folder) in defaults {
                let _ = conn.execute(
                    "INSERT OR IGNORE INTO bookmarks (url, title, icon, folder, created_at, visit_count)
                     VALUES (?1, ?2, ?3, ?4, ?5, 0);",
                    params![url, title, icon, folder, now],
                );
            }
        }
        Ok(())
    }

    /// Adds a new bookmark or updates existing one.
    pub fn add_bookmark(&self, url: &str, title: &str, icon: &str, folder: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO bookmarks (url, title, icon, folder, created_at, visit_count)
             VALUES (?1, ?2, ?3, ?4, ?5, 0)
             ON CONFLICT(url) DO UPDATE SET title = excluded.title, icon = excluded.icon, folder = excluded.folder;",
            params![url, title, icon, folder, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Removes a bookmark by its database ID.
    pub fn delete_bookmark(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM bookmarks WHERE id = ?1;", params![id])?;
        Ok(())
    }

    /// Removes a bookmark matching a given URL.
    pub fn delete_by_url(&self, url: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM bookmarks WHERE url = ?1;", params![url])?;
        Ok(())
    }

    /// Checks if a URL is currently bookmarked.
    pub fn is_bookmarked(&self, url: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM bookmarks WHERE url = ?1;",
            params![url],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Retrieves all stored bookmarks ordered by folder and title.
    pub fn get_all(&self) -> Result<Vec<BookmarkEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, url, title, icon, folder, created_at, visit_count
             FROM bookmarks
             ORDER BY folder ASC, title ASC;",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(BookmarkEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                icon: row.get(3)?,
                folder: row.get(4)?,
                created_at: row.get(5)?,
                visit_count: row.get(6)?,
            })
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            bookmarks.push(row?);
        }
        Ok(bookmarks)
    }

    /// Searches bookmarks by title, URL, or folder.
    pub fn search(&self, query: &str) -> Result<Vec<BookmarkEntry>> {
        let conn = self.conn.lock().unwrap();
        let like = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, url, title, icon, folder, created_at, visit_count
             FROM bookmarks
             WHERE title LIKE ?1 OR url LIKE ?1 OR folder LIKE ?1
             ORDER BY visit_count DESC, id DESC
             LIMIT 50;",
        )?;

        let rows = stmt.query_map([&like], |row| {
            Ok(BookmarkEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                icon: row.get(3)?,
                folder: row.get(4)?,
                created_at: row.get(5)?,
                visit_count: row.get(6)?,
            })
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            bookmarks.push(row?);
        }
        Ok(bookmarks)
    }

    /// Increments the visit count for a bookmarked URL.
    pub fn increment_visit(&self, url: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE bookmarks SET visit_count = visit_count + 1 WHERE url = ?1;",
            params![url],
        )?;
        Ok(())
    }

    /// Purges all bookmarks from the database.
    pub fn clear_all(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM bookmarks;", [])?;
        Ok(())
    }
}
