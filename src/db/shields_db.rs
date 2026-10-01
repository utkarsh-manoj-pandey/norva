// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/shields_db.rs
// Description: Dedicated SQLite database for Privacy Shields, Adblock metrics & rules.
// ============================================================================

use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Classification category for blocked telemetry/ad requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockCategory {
    Ad,
    Tracker,
    Fingerprinter,
    Cryptominer,
    TrackingParam,
}

impl BlockCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockCategory::Ad => "ad",
            BlockCategory::Tracker => "tracker",
            BlockCategory::Fingerprinter => "fingerprinter",
            BlockCategory::Cryptominer => "cryptominer",
            BlockCategory::TrackingParam => "tracking_param",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "tracker" => BlockCategory::Tracker,
            "fingerprinter" => BlockCategory::Fingerprinter,
            "cryptominer" => BlockCategory::Cryptominer,
            "tracking_param" => BlockCategory::TrackingParam,
            _ => BlockCategory::Ad,
        }
    }
}

/// A logged block event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedEvent {
    pub id: i64,
    pub category: BlockCategory,
    pub blocked_domain: String,
    pub page_domain: String,
    pub timestamp: String,
}

/// Real-time summary statistics of privacy shields.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShieldStats {
    pub total_ads_blocked: i64,
    pub total_trackers_blocked: i64,
    pub total_fingerprinters_blocked: i64,
    pub total_params_stripped: i64,
    pub total_all_time: i64,
    pub estimated_bandwidth_saved_kb: i64,
}

/// Dedicated Privacy Shields Database Manager.
///
/// I am writing this dedicated database manager because tracking protection
/// generates high-velocity event writes. By isolating shield counters, custom rules,
/// and block statistics in `privacy_shields.db`, we ensure the user's history and
/// bookmarks are never affected by heavy telemetry blocking activity.
#[derive(Clone)]
pub struct ShieldsDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl ShieldsDatabase {
    /// Opens or initializes the privacy shields database.
    ///
    /// I have written this initialization routine to enforce WAL mode and setup tables
    /// for block telemetry and shield configuration switches.
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
        )?;

        // Log of blocked network requests
        conn.execute(
            "CREATE TABLE IF NOT EXISTS blocked_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT NOT NULL,
                blocked_domain TEXT NOT NULL,
                page_domain TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );",
            [],
        )?;

        // Custom domain rules (whitelist / blacklist)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS custom_shield_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domain TEXT UNIQUE NOT NULL,
                action TEXT NOT NULL, -- 'block' or 'allow'
                created_at TEXT NOT NULL
            );",
            [],
        )?;

        // Shield switches (Adblock, Fingerprint Defense, Parameter Stripping, etc.)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS shield_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_blocked_cat ON blocked_events(category);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_blocked_time ON blocked_events(timestamp DESC);",
            [],
        )?;

        // Seed default shield configurations if not present
        let defaults = [
            ("adblock_enabled", "true"),
            ("tracker_protection", "true"),
            ("fingerprint_defense", "true"),
            ("strip_tracking_params", "true"),
            ("cosmetic_filtering", "true"),
        ];

        for (k, v) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO shield_settings (key, value) VALUES (?, ?)",
                params![k, v],
            )?;
        }

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Records a blocked ad, tracker, or stripped tracking parameter.
    ///
    /// I have written this recording method because showing users tangible, real-time proof
    /// of what NORVA has saved them from builds confidence and transparent security.
    pub fn record_block(&self, category: BlockCategory, blocked_domain: &str, page_domain: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO blocked_events (category, blocked_domain, page_domain, timestamp)
             VALUES (?, ?, ?, ?)",
            params![category.as_str(), blocked_domain, page_domain, now],
        )?;

        Ok(())
    }

    /// Retrieves aggregate shield metrics for the NORVA dashboard.
    ///
    /// I am calculating bandwidth savings at ~50KB per blocked ad/tracker asset,
    /// a standard empirical average for modern bloated web scripts.
    pub fn get_stats(&self) -> Result<ShieldStats> {
        let conn = self.conn.lock().unwrap();

        let ads: i64 = conn.query_row(
            "SELECT COUNT(*) FROM blocked_events WHERE category = 'ad'",
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        let trackers: i64 = conn.query_row(
            "SELECT COUNT(*) FROM blocked_events WHERE category = 'tracker'",
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        let fingerprinters: i64 = conn.query_row(
            "SELECT COUNT(*) FROM blocked_events WHERE category = 'fingerprinter'",
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        let params: i64 = conn.query_row(
            "SELECT COUNT(*) FROM blocked_events WHERE category = 'tracking_param'",
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        let total = ads + trackers + fingerprinters + params;
        let estimated_kb = total * 52; // average ~52KB saved per prevented payload

        Ok(ShieldStats {
            total_ads_blocked: ads,
            total_trackers_blocked: trackers,
            total_fingerprinters_blocked: fingerprinters,
            total_params_stripped: params,
            total_all_time: total,
            estimated_bandwidth_saved_kb: estimated_kb,
        })
    }

    /// Retrieves the most recent blocked events.
    pub fn get_recent_blocks(&self, limit: usize) -> Result<Vec<BlockedEvent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, category, blocked_domain, page_domain, timestamp
             FROM blocked_events
             ORDER BY id DESC
             LIMIT ?"
        )?;

        let rows = stmt.query_map([limit as i64], |row| {
            let cat_str: String = row.get(1)?;
            Ok(BlockedEvent {
                id: row.get(0)?,
                category: BlockCategory::from_str(&cat_str),
                blocked_domain: row.get(2)?,
                page_domain: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;

        let mut events = Vec::new();
        for ev in rows {
            if let Ok(e) = ev {
                events.push(e);
            }
        }
        Ok(events)
    }

    /// Retrieves a boolean shield setting.
    pub fn get_setting_bool(&self, key: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT value FROM shield_settings WHERE key = ?",
            [key],
            |r| r.get::<_, String>(0),
        ).map(|v| v == "true").unwrap_or(true)
    }

    /// Sets a shield setting.
    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO shield_settings (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }

    /// Clears the blocked events log.
    pub fn clear_events(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM blocked_events", [])?;
        conn.execute("VACUUM", [])?;
        Ok(())
    }
}
