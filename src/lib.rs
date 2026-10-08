// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/lib.rs
// Description: Core library crate for NORVA Browser subsystems and APIs.
// ============================================================================

#![allow(dead_code, unused_imports)]

/// Browser runtime orchestrator, event loop proxy, and IPC dispatchers.
///
/// I have written this module to coordinate between the window manager (Tao),
/// the WebKit layout engine (Wry), and native GTK desktop chrome widgets.
pub mod browser;

/// Configuration paths, directory isolator, and security mode definitions.
///
/// I have written this module to isolate all browser databases and cache directories
/// under strict POSIX user boundaries.
pub mod config;

/// Multi-database subsystem orchestrating five dedicated SQLite database files.
///
/// I have written this module because separating history, permissions, privacy shields,
/// settings, and cache/cookies into distinct SQLite databases prevents cross-subsystem corruption.
pub mod db;

/// Privacy shields, network adblock engine, hardware zero-trust arbiters, and cache purgers.
///
/// I have written this subsystem to sever connections to ad and telemetry networks
/// before requests leave the socket, and protect webcam, mic, and GPS hardware.
pub mod privacy;

/// Multi-engine search provider catalog, bang hotkey router, and query detection.
///
/// I have written this module to provide users with native search engine choice
/// across Google, Bing, DuckDuckGo, Yahoo, Ahmia (.onion), Startpage, Brave, GitHub, and Wikipedia.
pub mod search;

/// Executive cyber-dark UI templates, native GTK chrome header, and CSS design system.
///
/// I have written this module to provide a sleek, professional, modern interface
/// free of clutter, toy widgets, and amateurish styles.
pub mod ui;

pub use browser::NorvaBrowser;
pub use config::NorvaConfig;
