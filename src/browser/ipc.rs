// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/browser/ipc.rs
// Description: IPC message deserializer and command dispatcher between UI and Rust.
// ============================================================================

use crate::db::DatabaseManager;
use crate::privacy::CacheManager;
use crate::search::{QueryRouter, ResolvedTarget};
use serde::{Deserialize, Serialize};
use tao::event_loop::EventLoopProxy;

/// Events dispatched across the event loop proxy to control the main webview thread.
///
/// I have written this enum to maintain clean thread isolation: UI actions originating
/// inside the WebKit JavaScript sandbox are sent over a thread-safe message proxy to
/// mutate Tao window state, change tabs, or trigger navigations.
#[derive(Debug, Clone)]
pub enum NorvaUserEvent {
    Navigate(String),
    GoBack,
    GoForward,
    Reload,
    EvalScript(String),
    PageTitleChanged(String),
    PageUrlChanged(String),
    UpdateShieldCount(usize),
    NewTab,
    SwitchTab(usize),
    CloseTab(usize),
    SelectEngine { id: String, name: String },
}

/// Incoming JSON IPC payload structure from JavaScript `window.ipc.postMessage`.
///
/// I have written this struct with optional fields so that diverse UI interactions
/// (submitting searches, deleting history entries, adding custom engines, or clearing cache)
/// can reuse a single unified, type-safe deserializer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub action: String,
    pub query: Option<String>,
    pub id: Option<i64>,
    pub domain: Option<String>,
    pub name: Option<String>,
    pub template: Option<String>,
    pub bang: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
}

/// IPC Dispatcher handling client actions natively in Rust.
///
/// I have upgraded this dispatcher because user interactions occurring within the HTML5/CSS
/// frontend (such as clicking a speed dial, submitting the omnibox, resetting a device permission,
/// or purging cache memory) must be validated and executed natively in Rust with maximum security.
#[derive(Clone)]
pub struct IpcDispatcher {
    proxy: EventLoopProxy<NorvaUserEvent>,
    db: DatabaseManager,
    query_router: QueryRouter,
    cache_manager: CacheManager,
}

impl IpcDispatcher {
    /// Creates a new IPC dispatcher connected to the application subsystems.
    pub fn new(
        proxy: EventLoopProxy<NorvaUserEvent>,
        db: DatabaseManager,
        query_router: QueryRouter,
        cache_manager: CacheManager,
    ) -> Self {
        Self {
            proxy,
            db,
            query_router,
            cache_manager,
        }
    }

    /// Handles a raw IPC JSON string payload from JavaScript.
    ///
    /// I have written this parsing function with strict error handling so malformed
    /// or unsolicited client payloads fail safely without panicking.
    pub fn handle_raw_message(&self, raw: &str) {
        let msg: IpcMessage = match serde_json::from_str(raw) {
            Ok(m) => m,
            Err(_) => return,
        };

        match msg.action.as_str() {
            "navigate" => {
                if let Some(query) = msg.query {
                    let target = self.query_router.resolve(&query);
                    let url = match target {
                        ResolvedTarget::DirectUrl(u) => u,
                        ResolvedTarget::Search { target_url, .. } => target_url,
                        ResolvedTarget::InternalPage(p) => p,
                    };
                    let _ = self.proxy.send_event(NorvaUserEvent::Navigate(url));
                }
            }
            "back" => {
                let _ = self.proxy.send_event(NorvaUserEvent::GoBack);
            }
            "forward" => {
                let _ = self.proxy.send_event(NorvaUserEvent::GoForward);
            }
            "reload" => {
                let _ = self.proxy.send_event(NorvaUserEvent::Reload);
            }
            "new_tab" => {
                let _ = self.proxy.send_event(NorvaUserEvent::NewTab);
            }
            "delete_history" => {
                if let Some(id) = msg.id {
                    let _ = self.db.history.delete_entry(id);
                }
            }
            "clear_history" => {
                let _ = self.db.history.clear_all();
            }
            "purge_cache" => {
                let _ = self.cache_manager.purge_cache();
            }
            "set_default_engine" => {
                if let Some(id) = msg.id.map(|i| i.to_string()).or(msg.query) {
                    let _ = self.db.settings.set_default_engine(&id);
                }
            }
            "add_engine" => {
                if let (Some(name), Some(template), Some(bang)) = (msg.name, msg.template, msg.bang) {
                    let id = name.to_lowercase().replace(' ', "_");
                    let _ = self.db.settings.add_custom_engine(
                        &id,
                        &name,
                        &template,
                        &bang,
                        "Custom user-configured search engine",
                        false,
                    );
                }
            }
            "reset_permission" => {
                if let Some(domain) = msg.domain {
                    let _ = self.db.permissions.delete_domain(&domain);
                }
            }
            "clear_permissions_audit" => {
                let _ = self.db.permissions.clear_audits();
            }
            "set_preference" => {
                if let (Some(key), Some(value)) = (msg.key, msg.value) {
                    let _ = self.db.settings.set_preference(&key, &value);
                }
            }
            _ => {}
        }
    }
}
