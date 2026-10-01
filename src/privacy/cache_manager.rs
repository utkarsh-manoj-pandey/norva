// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/privacy/cache_manager.rs
// Description: Cache memory handling, disk cache quota monitoring & purging.
// ============================================================================

use crate::db::cache_cookies_db::CacheCookiesDatabase;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Cache Memory and Storage Handler.
///
/// I am writing this module because cache memory is a double-edged sword: while it
/// speeds up asset loading, super-cookies (ETag tracking, HSTS cache probing) exploit
/// cached HTTP resources to track users across browser restarts. NORVA gives users
/// immediate visibility into cache consumption and the ability to purge it with a single click.
#[derive(Clone)]
pub struct CacheManager {
    cache_cookies_db: Arc<CacheCookiesDatabase>,
    cache_dir: PathBuf,
}

impl CacheManager {
    /// Initializes the Cache Manager with the designated cache directory.
    pub fn new(cache_cookies_db: Arc<CacheCookiesDatabase>, cache_dir: PathBuf) -> Self {
        Self {
            cache_cookies_db,
            cache_dir,
        }
    }

    /// Calculates total disk cache size in bytes.
    ///
    /// I have written this recursive calculator so users can see exact megabytes
    /// of cached resources on their dashboard.
    pub fn get_cache_size_bytes(&self) -> u64 {
        Self::calc_dir_size(&self.cache_dir)
    }

    /// Helper function to recursively sum file sizes in a directory.
    fn calc_dir_size(path: &Path) -> u64 {
        if !path.exists() {
            return 0;
        }

        let mut total = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    total += Self::calc_dir_size(&p);
                } else if let Ok(meta) = entry.metadata() {
                    total += meta.len();
                }
            }
        }
        total
    }

    /// Purges all cached files from the cache directory and logs the audit event.
    ///
    /// I have written this purge function to wipe the disk cache cleanly and record
    /// the freed byte count in the `cache_cookies.db` SQLite database.
    pub fn purge_cache(&self) -> u64 {
        let size_before = self.get_cache_size_bytes();

        if self.cache_dir.exists() {
            if let Ok(entries) = fs::read_dir(&self.cache_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let _ = fs::remove_dir_all(&path);
                    } else {
                        let _ = fs::remove_file(&path);
                    }
                }
            }
        }

        let _ = self.cache_cookies_db.log_cleanup(
            "DiskCache",
            "User-initiated cache memory purge",
            size_before as i64,
        );

        size_before
    }
}
