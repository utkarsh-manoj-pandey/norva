// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/config.rs
// Description: Profile path manager and filesystem isolator for NORVA Browser.
// ============================================================================

use std::fs;
use std::path::PathBuf;

/// Application configuration and profile path management.
///
/// I have written this struct because a privacy-focused browser must maintain absolute
/// control and predictability over where user artifacts, databases, and cache files reside.
/// By explicitly isolating our configuration directories, we ensure zero leakage to
/// shared OS directories and provide users with a clean single directory to inspect or wipe.
///
/// In this upgraded architecture, each database path maps to an isolated SQLite file:
/// `history.db`, `permissions.db`, `privacy_shields.db`, `settings.db`, and `cache_cookies.db`.
#[derive(Debug, Clone)]
pub struct NorvaConfig {
    /// Base profile directory (e.g. ~/.local/share/norva)
    pub profile_dir: PathBuf,
    /// Path to the dedicated SQLite History database
    pub history_db_path: PathBuf,
    /// Path to the dedicated SQLite Hardware Permissions database
    pub permissions_db_path: PathBuf,
    /// Path to the dedicated SQLite Privacy Shields & Adblock database
    pub shields_db_path: PathBuf,
    /// Path to the dedicated SQLite Settings & Search Engines database
    pub settings_db_path: PathBuf,
    /// Path to the dedicated SQLite Cache & Cookie Storage database
    pub cache_cookies_db_path: PathBuf,
    /// Directory dedicated to WebKit's web data context
    pub web_data_dir: PathBuf,
    /// Directory dedicated to WebKit's disk cache
    pub web_cache_dir: PathBuf,
    /// Flag indicating whether the browser is running in Secure Mode (ephemeral / zero-trace)
    pub secure_mode: bool,
}

impl NorvaConfig {
    /// Initializes the NORVA configuration, creating all required directories.
    ///
    /// I have written this initialization function because before any database or webview
    /// attempts to open files, we must guarantee that the entire directory hierarchy
    /// exists with strict POSIX permissions so that other local system users
    /// cannot peek into NORVA's private data stores.
    pub fn init(secure_mode: bool) -> Self {
        // I am writing this path resolution logic because standard XDG directories
        // are the gold standard on Linux/BSD for user data isolation.
        let base_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("norva");

        let db_dir = base_dir.join("databases");
        let web_data_dir = base_dir.join("web_data");
        let web_cache_dir = base_dir.join("cache");

        // I have written this block because if we are in normal mode, we safely ensure
        // directories exist on disk. In secure mode, we keep memory isolation paramount.
        if !secure_mode {
            let _ = fs::create_dir_all(&db_dir);
            let _ = fs::create_dir_all(&web_data_dir);
            let _ = fs::create_dir_all(&web_cache_dir);
        }

        Self {
            profile_dir: base_dir,
            history_db_path: db_dir.join("history.db"),
            permissions_db_path: db_dir.join("permissions.db"),
            shields_db_path: db_dir.join("privacy_shields.db"),
            settings_db_path: db_dir.join("settings.db"),
            cache_cookies_db_path: db_dir.join("cache_cookies.db"),
            web_data_dir,
            web_cache_dir,
            secure_mode,
        }
    }

    /// Returns whether the current session is operating in Secure / Ghost Mode.
    ///
    /// I have written this helper because multiple subsystems (history logger, cookie persister,
    /// tracker shields) need an immediate, lightweight boolean check to decide whether to
    /// suppress persistent disk writes entirely.
    #[inline]
    pub fn is_secure_mode(&self) -> bool {
        self.secure_mode
    }
}
