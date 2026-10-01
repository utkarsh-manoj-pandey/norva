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
/// I am writing this struct because users should have total freedom over how they
/// navigate the web. Supporting major search engines (Google, Bing, DuckDuckGo, Yahoo,
/// and Ahmia for Tor .onion hidden services) with hotkey bangs (e.g. !g, !b, !ddg, !y, !ahmia)
/// makes navigation instantaneous while keeping the door wide open for custom user engines.
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
/// I have written this dedicated SQLite database (`settings.db`) because browser
/// preferences, search engine templates, and proxy toggles should be completely separated
/// from browsing history and shield counters. If a user deletes all their history or resets
/// their adblock logs, their carefully customized search engines and preferences remain intact.
#[derive(Clone)]
pub struct SettingsDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl SettingsDatabase {
    /// Opens or initializes the settings database.
    ///
    /// I have written this function to seed the primary search engines required by NORVA:
    /// Google, Microsoft Bing, DuckDuckGo, Yahoo, and Ahmia (for Tor .onion web searches),
    /// plus Startpage and Brave Search for extra privacy options.
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
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

        // Generic key-value preferences table
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
                "Privacy-focused search engine that doesn't track search history",
                1,
                1, // default
            ),
            (
                "google",
                "Google Search",
                "https://www.google.com/search?q={}",
                "!g",
                "World's most ubiquitous search engine index",
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
                "Specialized search engine designed to discover and index Tor .onion hidden services",
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
                "startpage",
                "Startpage",
                "https://www.startpage.com/sp/search?query={}",
                "!sp",
                "Google search results served with complete privacy protection",
                1,
                0,
            ),
        ];

        for (id, name, template, bang, desc, tor, is_def) in engines {
            conn.execute(
                "INSERT OR IGNORE INTO search_engines 
                 (id, name, query_template, bang, description, is_tor_compatible, is_default)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![id, name, template, bang, desc, tor, is_def],
            )?;
        }

        // Seed default preferences
        let default_prefs = [
            ("default_engine", "duckduckgo"),
            ("homepage", "norva://newtab"),
            ("secure_mode_on_startup", "false"),
            ("hardware_default_policy", "ask"),
            ("tor_socks5_proxy", "127.0.0.1:9050"),
            ("tor_proxy_enabled", "false"),
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

    /// Retrieves all registered search engines.
    ///
    /// I am writing this to dynamically render search engine selector dropdowns,
    /// start page badges, and search bar suggestions.
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
        for e in rows {
            if let Ok(item) = e {
                engines.push(item);
            }
        }
        Ok(engines)
    }

    /// Retrieves the current default search engine.
    ///
    /// I have written this method with a solid fallback: if for any reason
    /// the configured engine is missing, DuckDuckGo is safely returned.
    pub fn get_default_engine(&self) -> SearchEngineConfig {
        let conn = self.conn.lock().unwrap();
        let query_result = conn.query_row(
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
                    is_default: row.get::<_, i64>(6)? == 1,
                })
            },
        );

        query_result.unwrap_or_else(|_| SearchEngineConfig {
            id: "duckduckgo".to_string(),
            name: "DuckDuckGo".to_string(),
            query_template: "https://duckduckgo.com/?q={}".to_string(),
            bang: "!ddg".to_string(),
            description: "Privacy-focused search engine".to_string(),
            is_tor_compatible: true,
            is_default: true,
        })
    }

    /// Sets the default search engine by ID.
    ///
    /// I am writing this to atomically update both the `is_default` column in search_engines
    /// and the `default_engine` value in preferences.
    pub fn set_default_engine(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE search_engines SET is_default = 0", [])?;
        conn.execute("UPDATE search_engines SET is_default = 1 WHERE id = ?", [id])?;
        conn.execute(
            "INSERT OR REPLACE INTO preferences (key, value) VALUES ('default_engine', ?)",
            [id],
        )?;
        Ok(())
    }

    /// Looks up a search engine by its shortcut bang (e.g. "!g", "!b", "!ahmia").
    ///
    /// I have written this lookup function so queries like "!ahmia darknet"
    /// or "!b azure" are automatically routed to the requested search provider.
    pub fn get_engine_by_bang(&self, bang: &str) -> Option<SearchEngineConfig> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, query_template, bang, description, is_tor_compatible, is_default
             FROM search_engines
             WHERE LOWER(bang) = LOWER(?)
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

    /// Adds a new custom search engine.
    ///
    /// I am writing this method because future-proofing means allowing users to add
    /// their own internal corporate search, SearXNG instance, or custom Tor search engines.
    pub fn add_custom_engine(
        &self,
        id: &str,
        name: &str,
        query_template: &str,
        bang: &str,
        description: &str,
        is_tor: bool,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO search_engines (id, name, query_template, bang, description, is_tor_compatible, is_default)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
            params![id, name, query_template, bang, description, is_tor as i64],
        )?;
        Ok(())
    }

    /// Gets a preference value with a default fallback.
    pub fn get_pref(&self, key: &str, default_val: &str) -> String {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT value FROM preferences WHERE key = ?",
            [key],
            |r| r.get(0),
        ).unwrap_or_else(|_| default_val.to_string())
    }

    /// Sets a preference value.
    pub fn set_pref(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO preferences (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }
}
