// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/lib.rs
// Description: Core library crate for NORVA Browser subsystems and APIs.
// ============================================================================

#![allow(dead_code, unused_imports)]

pub mod browser;
pub mod config;
pub mod db;
pub mod privacy;
pub mod search;
pub mod ui;

pub use browser::NorvaBrowser;
pub use config::NorvaConfig;
