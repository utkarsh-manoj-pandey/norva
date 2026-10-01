// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/privacy/mod.rs
// Description: Privacy, anti-tracking, adblock, and hardware arbiter subsystem.
// ============================================================================

pub mod adblocker;
pub mod cache_manager;
pub mod cookie_manager;
pub mod permissions_manager;

pub use adblocker::{AdBlockEngine, FilterAction};
pub use cache_manager::CacheManager;
pub use cookie_manager::CookieManager;
pub use permissions_manager::PermissionsManager;
