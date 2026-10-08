// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/mod.rs
// Description: Multi-Database Manager coordinating separate SQLite databases per use case.
// ============================================================================

pub mod cache_cookies_db;
pub mod history_db;
pub mod permissions_db;
pub mod settings_db;
pub mod shields_db;

pub use cache_cookies_db::{CacheCookiesDatabase, CleanupAudit, CookiePolicy};
pub use history_db::{HistoryDatabase, HistoryEntry, HistoryStats};
pub use permissions_db::{PermissionAuditLog, PermissionPolicy, PermissionsDatabase, SitePermission};
pub use settings_db::{SearchEngineConfig, SettingsDatabase};
pub use shields_db::{BlockCategory, BlockedEvent, ShieldStats, ShieldsDatabase};

use crate::config::NorvaConfig;
use rusqlite::Result;
use std::sync::Arc;

/// Master Database Coordinator.
///
/// I have written this coordinator struct because a modern, high-security browser
/// must separate its data concerns across distinct SQLite databases.
/// Instead of a monolithic database where a crash or corruption during a heavy
/// adblock write could wipe out bookmarks or browsing history, NORVA establishes
/// five independent database files:
/// 1. `history.db`: Stores navigation history, timestamps, visit counts, and search logs.
/// 2. `permissions.db`: Stores internal/external camera, microphone, and location policies.
/// 3. `privacy_shields.db`: Stores adblock filters, blocked tracker telemetry, and defense switches.
/// 4. `settings.db`: Stores search engine templates (Google, Bing, DDG, Yahoo, Ahmia), bangs, and user preferences.
/// 5. `cache_cookies.db`: Stores storage policies, ephemeral session markers, and cleanup audit records.
///
/// In this upgraded release, I have unified high-performance WAL tuning, 64MB memory caches,
/// and busy timeouts across all five databases to guarantee zero lock contention and zero UI freezes.
#[derive(Clone)]
pub struct DatabaseManager {
    pub history: Arc<HistoryDatabase>,
    pub permissions: Arc<PermissionsDatabase>,
    pub shields: Arc<ShieldsDatabase>,
    pub settings: Arc<SettingsDatabase>,
    pub cache_cookies: Arc<CacheCookiesDatabase>,
}

impl DatabaseManager {
    /// Initializes all five dedicated databases according to the application configuration.
    ///
    /// I have written this initialization method to cleanly handle both regular on-disk
    /// operations and Secure Mode (where history and session storage operate strictly in-memory).
    pub fn init(config: &NorvaConfig) -> Result<Self> {
        let history = Arc::new(HistoryDatabase::new(
            &config.history_db_path,
            config.secure_mode,
        )?);

        let permissions = Arc::new(PermissionsDatabase::new(
            &config.permissions_db_path,
        )?);

        let shields = Arc::new(ShieldsDatabase::new(
            &config.shields_db_path,
        )?);

        let settings = Arc::new(SettingsDatabase::new(
            &config.settings_db_path,
        )?);

        let cache_cookies = Arc::new(CacheCookiesDatabase::new(
            &config.cache_cookies_db_path,
        )?);

        Ok(Self {
            history,
            permissions,
            shields,
            settings,
            cache_cookies,
        })
    }
}
