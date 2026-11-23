// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/passwords_db.rs
// Description: Dedicated SQLite database for secure local credentials vault.
// ============================================================================

use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A single saved password / credential entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: i64,
    pub domain: String,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Dedicated Passwords Database Manager.
#[derive(Clone)]
pub struct PasswordsDatabase {
    conn: Arc<Mutex<Connection>>,
    secure_mode: bool,
}

impl PasswordsDatabase {
    /// Opens or creates the dedicated passwords database.
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
            "CREATE TABLE IF NOT EXISTS credentials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domain TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(domain, username)
            );",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_credentials_domain ON credentials(domain);",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            secure_mode,
        })
    }

    /// Saves or updates a credential.
    pub fn save_credential(
        &self,
        domain: &str,
        username: &str,
        password: &str,
        notes: Option<&str>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO credentials (domain, username, password, notes, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)
             ON CONFLICT(domain, username) DO UPDATE SET
                password = excluded.password,
                notes = excluded.notes,
                updated_at = excluded.updated_at;",
            params![domain, username, password, notes, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Retrieves all credentials for a specific domain.
    pub fn get_for_domain(&self, domain: &str) -> Result<Vec<PasswordEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, domain, username, password, notes, created_at, updated_at
             FROM credentials
             WHERE domain = ?1
             ORDER BY updated_at DESC;",
        )?;

        let rows = stmt.query_map([domain], |row| {
            Ok(PasswordEntry {
                id: row.get(0)?,
                domain: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                notes: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    /// Retrieves all saved credentials.
    pub fn get_all(&self) -> Result<Vec<PasswordEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, domain, username, password, notes, created_at, updated_at
             FROM credentials
             ORDER BY domain ASC, username ASC;",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(PasswordEntry {
                id: row.get(0)?,
                domain: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                notes: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    /// Deletes a credential by ID.
    pub fn delete(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM credentials WHERE id = ?1;", params![id])?;
        Ok(())
    }

    /// Searches credentials by domain or username.
    pub fn search(&self, query: &str) -> Result<Vec<PasswordEntry>> {
        let conn = self.conn.lock().unwrap();
        let like = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, domain, username, password, notes, created_at, updated_at
             FROM credentials
             WHERE domain LIKE ?1 OR username LIKE ?1
             ORDER BY domain ASC;",
        )?;

        let rows = stmt.query_map([&like], |row| {
            Ok(PasswordEntry {
                id: row.get(0)?,
                domain: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                notes: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    /// Purges all stored credentials.
    pub fn clear_all(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM credentials;", [])?;
        Ok(())
    }
}
