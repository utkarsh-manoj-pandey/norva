// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/search/query_router.rs
// Description: Omnibox input parser, Bang command dispatcher & search query routing.
// ============================================================================

use crate::db::settings_db::SettingsDatabase;
use std::sync::Arc;

/// Destination resolved from omnibox input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedTarget {
    /// Navigate directly to a fully qualified URL
    DirectUrl(String),
    /// Perform a web search on an engine with a query string
    Search {
        engine_name: String,
        query: String,
        target_url: String,
    },
    /// Internal NORVA system page
    InternalPage(String),
}

impl ResolvedTarget {
    /// Extracts the resolved destination as a navigable URL string.
    pub fn to_url(&self) -> String {
        match self {
            ResolvedTarget::DirectUrl(u) => u.clone(),
            ResolvedTarget::Search { target_url, .. } => target_url.clone(),
            ResolvedTarget::InternalPage(p) => p.clone(),
        }
    }

    /// Consumes the target and returns the destination URL string.
    pub fn into_url(self) -> String {
        match self {
            ResolvedTarget::DirectUrl(u) => u,
            ResolvedTarget::Search { target_url, .. } => target_url,
            ResolvedTarget::InternalPage(p) => p,
        }
    }
}

/// Query Router for the NORVA Omnibox.
///
/// I am writing this router because the browser address bar is the most frequent
/// point of human-computer interaction. A smart, privacy-centric omnibox must
/// effortlessly differentiate between direct domain visits, internal system panels
/// (like `norva://shields`), bang shortcuts (`!ahmia hidden services`), and standard queries.
#[derive(Clone)]
pub struct QueryRouter {
    settings_db: Arc<SettingsDatabase>,
}

impl QueryRouter {
    /// Creates a new query router linked to the settings database.
    pub fn new(settings_db: Arc<SettingsDatabase>) -> Self {
        Self { settings_db }
    }

    /// Resolves raw omnibox text into a navigation target.
    ///
    /// I have written this routing logic to inspect prefixes, bang hotkeys, and domain syntax:
    /// 1. If it starts with `norva://`, route to the internal page.
    /// 2. If it starts with a bang (e.g. `!ahmia`, `!g`, `!b`, `!ddg`, `!y`), route to that engine.
    /// 3. If it looks like a valid URL or IP address, append `https://` if missing and navigate.
    /// 4. Otherwise, execute a search using the user's default configured search engine.
    pub fn resolve(&self, input: &str) -> ResolvedTarget {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return ResolvedTarget::InternalPage("norva://newtab".to_string());
        }

        // 1. Internal NORVA pages
        if trimmed.starts_with("norva://") {
            return ResolvedTarget::InternalPage(trimmed.to_string());
        }

        // 2. Bang commands (e.g., "!g rust programming", "!ahmia hidden marketplace", "!b cloud")
        if trimmed.starts_with('!') {
            let mut parts = trimmed.splitn(2, ' ');
            let bang = parts.next().unwrap_or("");
            let query = parts.next().unwrap_or("").trim();

            // Special alias "!onion" -> Ahmia
            let effective_bang = if bang.eq_ignore_ascii_case("!onion") {
                "!ahmia"
            } else {
                bang
            };

            if let Some(engine) = self.settings_db.get_engine_by_bang(effective_bang) {
                let target_url = if query.is_empty() {
                    // Navigate to engine homepage
                    engine.query_template.split('?').next().unwrap_or(&engine.query_template).to_string()
                } else {
                    engine.query_template.replace("{}", &urlencoding_simple(query))
                };

                return ResolvedTarget::Search {
                    engine_name: engine.name,
                    query: query.to_string(),
                    target_url,
                };
            }
        }

        // 3. Check if input is a valid URL or host
        if is_probable_url(trimmed) {
            let url = if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("ftp://") {
                trimmed.to_string()
            } else {
                format!("https://{}", trimmed)
            };
            return ResolvedTarget::DirectUrl(url);
        }

        // 4. Fallback to default configured search engine
        let default_engine = self.settings_db.get_default_engine();
        let target_url = default_engine.query_template.replace("{}", &urlencoding_simple(trimmed));

        ResolvedTarget::Search {
            engine_name: default_engine.name,
            query: trimmed.to_string(),
            target_url,
        }
    }
}

/// Determines whether an input string is a domain name, IP, or URL rather than a search phrase.
fn is_probable_url(input: &str) -> bool {
    if input.starts_with("http://") || input.starts_with("https://") {
        return true;
    }

    if input.contains(' ') {
        return false;
    }

    if input.starts_with("localhost:") || input == "localhost" {
        return true;
    }

    // IPv4 address check
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return true;
    }

    // Standard top-level domain extension check
    if input.contains('.') {
        if let Some(suffix) = input.split('.').last() {
            let s = suffix.split('/').next().unwrap_or(suffix);
            let common_tlds = [
                "com", "org", "net", "io", "dev", "ai", "app", "co", "uk", "de",
                "in", "fr", "ca", "xyz", "onion", "me", "info", "is", "fi", "rs"
            ];
            if common_tlds.contains(&s.to_lowercase().as_str()) {
                return true;
            }
        }
    }

    false
}

/// Simple percent encoder for query parameters.
fn urlencoding_simple(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => {
                encoded.push('+');
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}
