// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/privacy/cookie_manager.rs
// Description: Cookie isolation, per-domain storage policies & anti-tracking.
// ============================================================================

use crate::db::cache_cookies_db::CacheCookiesDatabase;
use std::sync::Arc;

/// Cookie & Storage Management Controller.
///
/// I have written this manager because cookies remain the most prevalent mechanism for
/// cross-site user re-identification. While essential for session authentication (like
/// staying logged into an account), third-party tracking cookies serve solely to build
/// surveillance profiles. NORVA enforces strict first-party partitioning and provides
/// users with a 1-click mechanism to purge cookies on demand.
#[derive(Clone)]
pub struct CookieManager {
    cache_cookies_db: Arc<CacheCookiesDatabase>,
}

impl CookieManager {
    /// Creates a new cookie manager linked to the cache/cookies database.
    pub fn new(cache_cookies_db: Arc<CacheCookiesDatabase>) -> Self {
        Self { cache_cookies_db }
    }

    /// Evaluates if a domain should have its cookies cleared upon closing the tab/window.
    ///
    /// I have written this check so that sensitive web domains or ephemeral sessions
    /// are automatically purged as soon as the session terminates.
    pub fn should_clear_on_exit(&self, domain: &str) -> bool {
        let policy = self.cache_cookies_db.get_cookie_policy(domain);
        policy.clear_on_exit
    }

    /// Generates a client-side JavaScript snippet to purge document cookies for the active domain.
    ///
    /// I have written this helper because WebKit's DOM context allows instantaneous
    /// neutralization of active session cookies, local storage, and session storage from within the webview.
    pub fn get_cookie_wipe_script() -> &'static str {
        r#"
        (function() {
            try {
                const cookies = document.cookie.split(";");
                for (let i = 0; i < cookies.length; i++) {
                    const cookie = cookies[i];
                    const eqPos = cookie.indexOf("=");
                    const name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
                    document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT;path=/";
                    document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT;path=/;domain=" + window.location.hostname;
                }
                localStorage.clear();
                sessionStorage.clear();
            } catch(e) {}
        })();
        "#
    }
}
