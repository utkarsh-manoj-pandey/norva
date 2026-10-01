// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/browser/mod.rs
// Description: Browser application coordinator and runtime state.
// ============================================================================

pub mod app;
pub mod ipc;
pub mod state;

pub use app::NorvaBrowser;
pub use ipc::{IpcDispatcher, NorvaUserEvent};
pub use state::BrowserState;
