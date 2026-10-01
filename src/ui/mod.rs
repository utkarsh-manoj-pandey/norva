// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/ui/mod.rs
// Description: UI asset renderers, CSS design system, and injected HUD overlay.
// ============================================================================

pub mod assets;
pub mod gtk_chrome;
pub mod hud_script;

pub use assets::*;
pub use gtk_chrome::NorvaGtkChrome;
pub use hud_script::get_hud_injection_script;
