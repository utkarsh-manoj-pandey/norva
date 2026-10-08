// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/search/engine.rs
// Description: Search engine model with native support for Google, Bing, DDG, Yahoo, and Ahmia (.onion).
// ============================================================================

use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a configured search engine in NORVA.
///
/// I have written this struct because search engine flexibility is a cornerstone of NORVA.
/// Users should never be locked into a single search monopoly. We provide first-class
/// native integration for Google, Microsoft Bing, DuckDuckGo, Yahoo Search, Ahmia
/// (the premier clearnet search engine index for Tor .onion hidden services), Startpage,
/// Brave Search, GitHub, and Wikipedia, while allowing unlimited custom search providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub id: String,
    pub name: String,
    pub query_template: String,
    pub bang: String,
    pub description: String,
    pub is_tor_compatible: bool,
}

impl SearchEngine {
    /// Formats a search query string into a fully qualified destination URL.
    ///
    /// I have written this helper to safely percent-encode user queries so spaces,
    /// punctuation, and non-ASCII Unicode characters are cleanly handled.
    pub fn build_search_url(&self, query: &str) -> String {
        let encoded_query = urlencoding_simple(query);
        self.query_template.replace("{}", &encoded_query)
    }

    /// Attempts to extract the search term and engine from a visited URL.
    ///
    /// I have upgraded this detection function so that when a user browses, NORVA
    /// can identify searches across Google, Bing, Yahoo, DDG, Ahmia, Startpage, Brave,
    /// GitHub, and Wikipedia, allowing the history vault to categorize and index queries intelligently.
    pub fn detect_search(url_str: &str) -> Option<(String, String)> {
        let url = Url::parse(url_str).ok()?;
        let host = url.host_str()?.to_lowercase();

        if host.contains("google.") && url.path().starts_with("/search") {
            let q = url.query_pairs().find(|(k, _)| k == "q")?.1.to_string();
            return Some((q, "Google Search".to_string()));
        }

        if host.contains("bing.com") && url.path().starts_with("/search") {
            let q = url.query_pairs().find(|(k, _)| k == "q")?.1.to_string();
            return Some((q, "Microsoft Bing".to_string()));
        }

        if host.contains("duckduckgo.com") {
            let q = url.query_pairs().find(|(k, _)| k == "q")?.1.to_string();
            return Some((q, "DuckDuckGo".to_string()));
        }

        if host.contains("yahoo.com") && url.path().starts_with("/search") {
            let q = url.query_pairs().find(|(k, _)| k == "p")?.1.to_string();
            return Some((q, "Yahoo Search".to_string()));
        }

        if host.contains("ahmia.fi") && url.path().starts_with("/search") {
            let q = url.query_pairs().find(|(k, _)| k == "q")?.1.to_string();
            return Some((q, "Ahmia (.onion Search)".to_string()));
        }

        if host.contains("startpage.com") {
            let q = url.query_pairs().find(|(k, _)| k == "query" || k == "q")?.1.to_string();
            return Some((q, "Startpage".to_string()));
        }

        if host.contains("search.brave.com") {
            let q = url.query_pairs().find(|(k, _)| k == "q")?.1.to_string();
            return Some((q, "Brave Search".to_string()));
        }

        if host.contains("github.com") && url.path().starts_with("/search") {
            let q = url.query_pairs().find(|(k, _)| k == "q")?.1.to_string();
            return Some((q, "GitHub Search".to_string()));
        }

        if host.contains("wikipedia.org") {
            let q = url.query_pairs().find(|(k, _)| k == "search")?.1.to_string();
            return Some((q, "Wikipedia".to_string()));
        }

        None
    }
}

/// Simple percent encoder for query parameters.
///
/// I have written this lightweight character encoder to avoid pulling extra heavy
/// external crates for basic query encoding, ensuring fast execution and small binaries.
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
