// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/browser/state.rs
// Description: Thread-safe runtime session state for the NORVA browser.
// ============================================================================

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

/// Thread-safe runtime session state.
///
/// I am writing this struct because a modern asynchronous web browser operates across
/// multiple threads: network protocol interceptors, IPC message handlers, and the main
/// GUI event loop all require synchronized, thread-safe access to the active URL,
/// page title, and live privacy shield counters without deadlocking the UI.
#[derive(Clone)]
pub struct BrowserState {
    pub current_url: Arc<RwLock<String>>,
    pub current_title: Arc<RwLock<String>>,
    pub secure_mode: bool,
    pub shields_active: Arc<AtomicBool>,
    pub session_blocked_count: Arc<AtomicUsize>,
}

impl BrowserState {
    /// Creates a new browser state container.
    ///
    /// I have written this constructor to initialize default states: shields enabled by default,
    /// current URL set to the NORVA newtab start page, and zeroed session counters.
    pub fn new(secure_mode: bool) -> Self {
        Self {
            current_url: Arc::new(RwLock::new("norva://newtab".to_string())),
            current_title: Arc::new(RwLock::new("NORVA Browser".to_string())),
            secure_mode,
            shields_active: Arc::new(AtomicBool::new(true)),
            session_blocked_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Updates the active URL.
    pub fn set_url(&self, url: &str) {
        if let Ok(mut lock) = self.current_url.write() {
            *lock = url.to_string();
        }
    }

    /// Gets a copy of the active URL.
    pub fn get_url(&self) -> String {
        self.current_url.read().map(|r| r.clone()).unwrap_or_else(|_| "norva://newtab".to_string())
    }

    /// Updates the active page title.
    pub fn set_title(&self, title: &str) {
        if let Ok(mut lock) = self.current_title.write() {
            *lock = title.to_string();
        }
    }

    /// Gets a copy of the active page title.
    pub fn get_title(&self) -> String {
        self.current_title.read().map(|r| r.clone()).unwrap_or_else(|_| "NORVA".to_string())
    }

    /// Increments the session blocked ad/tracker tally atomically.
    pub fn inc_blocked(&self) -> usize {
        self.session_blocked_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Gets the current session blocked count.
    pub fn get_blocked_count(&self) -> usize {
        self.session_blocked_count.load(Ordering::SeqCst)
    }
}
