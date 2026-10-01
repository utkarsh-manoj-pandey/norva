// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/search/mod.rs
// Description: Multi-engine search subsystem supporting Google, Bing, DDG, Yahoo, and Ahmia (.onion).
// ============================================================================

pub mod engine;
pub mod query_router;

pub use engine::SearchEngine;
pub use query_router::{QueryRouter, ResolvedTarget};
