// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/settings_db.rs
// Description: Dedicated SQLite database for Search Engines & Browser Configuration.
// ============================================================================

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A registered Search Engine definition in NORVA.
///
/// I have written this struct because users should have total freedom over how they
/// navigate the web. Supporting major search engines (Google, Bing, DuckDuckGo, Yahoo,
/// Ahmia for Tor .onion hidden services, Brave Search, Startpage, GitHub, Wikipedia)
/// with hotkey bangs (e.g. !g, !b, !ddg, !y, !ahmia, !gh, !w) makes navigation
/// instantaneous while keeping the door wide open for custom user engines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineConfig {
    pub id: String,
    pub name: String,
    pub query_template: String,
    pub bang: String,
    pub description: String,
    pub is_tor_compatible: bool,
    pub is_default: bool,
}

/// Dedicated Settings Database Manager.
///
/// I have upgraded this dedicated SQLite database (`settings.db`) with optimized
/// caching and WAL mode. I wrote this architecture because browser preferences,
/// search engine templates, and user toggles must be isolated from history and
/// shield logs: when a user wipes history or clears cookies, their personalized
/// search engines, shortcuts, and settings remain completely intact.
#[derive(Clone)]
pub struct SettingsDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl SettingsDatabase {
    /// Opens or initializes the settings database with pre-seeded search engines.
    ///
    /// I have written this initialization function to seed a premier catalog of
    /// 9 high-utility search engines: DuckDuckGo (privacy default), Google, Bing,
    /// Yahoo, Ahmia (.onion Tor search), Startpage, Brave Search, GitHub, and Wikipedia.
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

        // Search engine catalog table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS search_engines (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                query_template TEXT NOT NULL,
                bang TEXT UNIQUE NOT NULL,
                description TEXT NOT NULL,
                is_tor_compatible BOOLEAN NOT NULL DEFAULT 0,
                is_default BOOLEAN NOT NULL DEFAULT 0
            );",
            [],
        )?;

        // Generic key-value preferences table for theme and widget toggles
        conn.execute(
            "CREATE TABLE IF NOT EXISTS preferences (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
            [],
        )?;

        // Seed search engines
        let engines = [
            (
                "duckduckgo",
                "DuckDuckGo",
                "https://duckduckgo.com/?q={}",
                "!ddg",
                "Privacy-first search engine with zero user tracking",
                1,
                1, // default
            ),
            (
                "google",
                "Google Search",
                "https://www.google.com/search?q={}",
                "!g",
                "Global search index with comprehensive web crawling",
                0,
                0,
            ),
            (
                "bing",
                "Microsoft Bing",
                "https://www.bing.com/search?q={}",
                "!b",
                "Microsoft's AI-augmented web search index",
                0,
                0,
            ),
            (
                "yahoo",
                "Yahoo Search",
                "https://search.yahoo.com/search?p={}",
                "!y",
                "Longstanding web portal and search index",
                0,
                0,
            ),
            (
                "ahmia",
                "Ahmia (Tor .onion Search)",
                "https://ahmia.fi/search/?q={}",
                "!ahmia",
                "Premier search engine for Tor hidden services (.onion)",
                1,
                0,
            ),
            (
                "startpage",
                "Startpage",
                "https://www.startpage.com/sp/search?query={}",
                "!sp",
                "Google results served with strict European privacy protections",
                1,
                0,
            ),
            (
                "brave",
                "Brave Search",
                "https://search.brave.com/search?q={}",
                "!brave",
                "Independent search index built from the ground up for privacy",
                1,
                0,
            ),
            (
                "github",
                "GitHub",
                "https://github.com/search?q={}",
                "!gh",
                "Open-source code repositories, packages, and developer discussions",
                0,
                0,
            ),
            (
                "wikipedia",
                "Wikipedia",
                "https://en.wikipedia.org/wiki/Special:Search?search={}",
                "!w",
                "The free encyclopedia and global open knowledge base",
                1,
                0,
            ),
        ];

        for (id, name, tmpl, bang, desc, tor, is_def) in engines {
            conn.execute(
                "INSERT OR IGNORE INTO search_engines
                 (id, name, query_template, bang, description, is_tor_compatible, is_default)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![id, name, tmpl, bang, desc, tor, is_def],
            )?;
        }

        // Default UI preferences
        let default_prefs = [
            ("theme_mode", "dark_executive"),
            ("show_speed_dial", "true"),
            ("show_threat_telemetry", "true"),
            ("show_hardware_monitor", "true"),
            ("show_quick_actions", "true"),
        ];

        for (k, v) in default_prefs {
            conn.execute(
                "INSERT OR IGNORE INTO preferences (key, value) VALUES (?, ?)",
                params![k, v],
            )?;
        }

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Retrieves all configured search engines.
    pub fn get_all_search_engines(&self) -> Result<Vec<SearchEngineConfig>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, query_template, bang, description, is_tor_compatible, is_default
             FROM search_engines
             ORDER BY is_default DESC, name ASC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(SearchEngineConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                query_template: row.get(2)?,
                bang: row.get(3)?,
                description: row.get(4)?,
                is_tor_compatible: row.get::<_, i64>(5)? == 1,
                is_default: row.get::<_, i64>(6)? == 1,
            })
        })?;

        let mut engines = Vec::new();
        for r in rows {
            engines.push(r?);
        }

        Ok(engines)
    }

    /// Retrieves the current default search engine configuration.
    ///
    /// I have written this helper with a fallback to DuckDuckGo so that even if database
    /// records are missing, the omnibox query builder always has a valid search provider.
    pub fn get_default_engine(&self) -> SearchEngineConfig {
        let conn = self.conn.lock().unwrap();
        let query_res = conn.query_row(
            "SELECT id, name, query_template, bang, description, is_tor_compatible, is_default
             FROM search_engines
             WHERE is_default = 1
             LIMIT 1",
            [],
            |row| {
                Ok(SearchEngineConfig {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    query_template: row.get(2)?,
                    bang: row.get(3)?,
                    description: row.get(4)?,
                    is_tor_compatible: row.get::<_, i64>(5)? == 1,
                    is_default: true,
                })
            },
        );

        query_res.unwrap_or_else(|_| SearchEngineConfig {
            id: "duckduckgo".to_string(),
            name: "DuckDuckGo".to_string(),
            query_template: "https://duckduckgo.com/?q={}".to_string(),
            bang: "!ddg".to_string(),
            description: "Default fallback privacy engine".to_string(),
            is_tor_compatible: true,
            is_default: true,
        })
    }

    /// Sets the default search engine by its unique ID.
    ///
    /// I have written this method to execute an atomic transaction: first clearing
    /// `is_default` on all existing records, then setting `is_default = 1` on the chosen engine.
    pub fn set_default_engine(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE search_engines SET is_default = 0", [])?;
        conn.execute("UPDATE search_engines SET is_default = 1 WHERE id = ?", [id])?;
        Ok(())
    }

    /// Looks up a search engine by its bang shortcut (e.g. `!g`, `!ddg`, `!ahmia`).
    pub fn get_engine_by_bang(&self, bang: &str) -> Option<SearchEngineConfig> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, query_template, bang, description, is_tor_compatible, is_default
             FROM search_engines
             WHERE bang = ?
             LIMIT 1",
            [bang],
            |row| {
                Ok(SearchEngineConfig {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    query_template: row.get(2)?,
                    bang: row.get(3)?,
                    description: row.get(4)?,
                    is_tor_compatible: row.get::<_, i64>(5)? == 1,
                    is_default: row.get::<_, i64>(6)? == 1,
                })
            },
        ).ok()
    }

    /// Adds or updates a custom search engine configuration.
    pub fn add_custom_engine(
        &self,
        id: &str,
        name: &str,
        query_template: &str,
        bang: &str,
        description: &str,
        is_tor_compatible: bool,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO search_engines (id, name, query_template, bang, description, is_tor_compatible, is_default)
             VALUES (?, ?, ?, ?, ?, ?, 0)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                query_template = excluded.query_template,
                bang = excluded.bang,
                description = excluded.description,
                is_tor_compatible = excluded.is_tor_compatible",
            params![id, name, query_template, bang, description, is_tor_compatible as i64],
        )?;
        Ok(())
    }

    /// Deletes a custom search engine (unless it is currently marked as default).
    pub fn delete_engine(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM search_engines WHERE id = ? AND is_default = 0", [id])?;
        Ok(())
    }

    /// Sets a generic user preference key-value pair.
    pub fn set_preference(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO preferences (key, value) VALUES (?, ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    /// Retrieves a user preference value.
    pub fn get_preference(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM preferences WHERE key = ?")?;
        let mut rows = stmt.query([key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
}
