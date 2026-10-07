// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: tests/norva_tests.rs
// Description: Comprehensive test suite for NORVA privacy subsystems & databases.
// ============================================================================

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

// Test directory setup
fn get_test_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("norva_test").join(name);
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::create_dir_all(&dir);
    dir
}

#[test]
fn test_history_sqlite_database() {
    let dir = get_test_dir("history");
    let db_path = dir.join("history.db");

    let history_db = norva::db::history_db::HistoryDatabase::new(&db_path, false)
        .expect("History DB initialization failed");

    // 1. Record regular page visit
    history_db
        .record_visit("https://rust-lang.org", "Rust Language", None, None)
        .expect("Failed to record visit");

    // 2. Record search visit
    history_db
        .record_visit(
            "https://duckduckgo.com/?q=privacy+browser",
            "DuckDuckGo Search",
            Some("privacy browser"),
            Some("DuckDuckGo"),
        )
        .expect("Failed to record search visit");

    // 3. Sequential deduplication: visiting same URL increments count
    history_db
        .record_visit("https://rust-lang.org", "Rust Language Home", None, None)
        .expect("Failed to re-record visit");

    let recent = history_db.get_recent(10).expect("Failed to get recent");
    assert_eq!(recent.len(), 2);

    let rust_entry = recent.iter().find(|e| e.url == "https://rust-lang.org").unwrap();
    assert_eq!(rust_entry.visit_count, 2);

    let ddg_entry = recent.iter().find(|e| e.url.contains("duckduckgo")).unwrap();
    assert_eq!(ddg_entry.search_query.as_deref(), Some("privacy browser"));
    assert_eq!(ddg_entry.search_engine.as_deref(), Some("DuckDuckGo"));

    // 4. Test history search
    let search_results = history_db.search("privacy", 10).expect("Search failed");
    assert_eq!(search_results.len(), 1);

    // 5. Test stats
    let stats = history_db.get_stats().expect("Stats failed");
    assert_eq!(stats.total_entries, 2);
    assert_eq!(stats.total_searches, 1);

    // 6. Test clear all
    history_db.clear_all().expect("Clear failed");
    let after_clear = history_db.get_recent(10).unwrap();
    assert_eq!(after_clear.len(), 0);
}

#[test]
fn test_permissions_hardware_and_location_database() {
    let dir = get_test_dir("permissions");
    let db_path = dir.join("permissions.db");

    let perm_db = norva::db::permissions_db::PermissionsDatabase::new(&db_path)
        .expect("Permissions DB initialization failed");

    // Default policy should be None for new domain
    let policy = perm_db.get_domain_policy("meet.google.com").unwrap();
    assert!(policy.is_none());

    // Set hardware permissions: Allow Camera & Microphone, Block Location
    perm_db
        .set_policy(
            "meet.google.com",
            norva::db::permissions_db::PermissionPolicy::Allow,
            norva::db::permissions_db::PermissionPolicy::Allow,
            norva::db::permissions_db::PermissionPolicy::Block,
        )
        .expect("Failed to set permission policy");

    let saved = perm_db.get_domain_policy("meet.google.com").unwrap().unwrap();
    assert_eq!(saved.camera, norva::db::permissions_db::PermissionPolicy::Allow);
    assert_eq!(saved.microphone, norva::db::permissions_db::PermissionPolicy::Allow);
    assert_eq!(saved.location, norva::db::permissions_db::PermissionPolicy::Block);

    // Log audit events
    perm_db
        .log_audit("meet.google.com", "camera", "allowed")
        .expect("Failed to log audit");
    perm_db
        .log_audit("meet.google.com", "location", "blocked")
        .expect("Failed to log audit");

    let audits = perm_db.get_recent_audits(10).unwrap();
    assert_eq!(audits.len(), 2);
    assert_eq!(audits[0].decision, "blocked");
    assert_eq!(audits[1].decision, "allowed");
}

#[test]
fn test_privacy_shields_and_adblock_engine() {
    let dir = get_test_dir("shields");
    let db_path = dir.join("shields.db");

    let shields_db = Arc::new(
        norva::db::shields_db::ShieldsDatabase::new(&db_path)
            .expect("Shields DB initialization failed"),
    );

    let adblock = norva::privacy::adblocker::AdBlockEngine::new(shields_db.clone());

    // 1. Test advertising network blocking
    let action_ad = adblock.evaluate_url("https://doubleclick.net/ad/serve.js");
    match action_ad {
        norva::privacy::adblocker::FilterAction::Block { category, domain } => {
            assert_eq!(category, norva::db::shields_db::BlockCategory::Ad);
            assert_eq!(domain, "doubleclick.net");
        }
        _ => panic!("Expected doubleclick.net to be blocked"),
    }

    // 2. Test tracker telemetry blocking
    let action_tracker = adblock.evaluate_url("https://analytics.google.com/collect");
    match action_tracker {
        norva::privacy::adblocker::FilterAction::Block { category, domain } => {
            assert_eq!(category, norva::db::shields_db::BlockCategory::Tracker);
            assert_eq!(domain, "analytics.google.com");
        }
        _ => panic!("Expected Google Analytics to be blocked"),
    }

    // 3. Test URL parameter stripping (fbclid, gclid, utm_source)
    let dirty_url = "https://example.com/article?utm_source=twitter&fbclid=IwAR123&keep=important";
    let action_sanitize = adblock.evaluate_url(dirty_url);
    match action_sanitize {
        norva::privacy::adblocker::FilterAction::SanitizeUrl(clean_url) => {
            assert!(!clean_url.contains("fbclid"));
            assert!(!clean_url.contains("utm_source"));
            assert!(clean_url.contains("keep=important"));
        }
        _ => panic!("Expected URL tracking parameters to be stripped"),
    }

    // 4. Test safe URL allowance
    let action_safe = adblock.evaluate_url("https://wikipedia.org/wiki/Rust_(programming_language)");
    assert_eq!(action_safe, norva::privacy::adblocker::FilterAction::Allow);

    // 5. Verify metrics were logged to shields.db
    let stats = shields_db.get_stats().expect("Stats query failed");
    assert!(stats.total_ads_blocked >= 1);
    assert!(stats.total_trackers_blocked >= 1);
    assert!(stats.total_params_stripped >= 1);
    assert!(stats.estimated_bandwidth_saved_kb > 0);
}

#[test]
fn test_search_engines_and_query_router() {
    let dir = get_test_dir("settings");
    let db_path = dir.join("settings.db");

    let settings_db = Arc::new(
        norva::db::settings_db::SettingsDatabase::new(&db_path)
            .expect("Settings DB initialization failed"),
    );

    let router = norva::search::query_router::QueryRouter::new(settings_db.clone());

    // 1. Google bang (!g)
    let target_g = router.resolve("!g quantum computing");
    match target_g {
        norva::search::query_router::ResolvedTarget::Search { engine_name, query, target_url } => {
            assert_eq!(engine_name, "Google Search");
            assert_eq!(query, "quantum computing");
            assert!(target_url.contains("google.com/search?q=quantum+computing"));
        }
        _ => panic!("Expected Google Search target"),
    }

    // 2. Microsoft Bing bang (!b)
    let target_b = router.resolve("!b machine learning");
    match target_b {
        norva::search::query_router::ResolvedTarget::Search { engine_name, query, target_url } => {
            assert_eq!(engine_name, "Microsoft Bing");
            assert_eq!(query, "machine learning");
            assert!(target_url.contains("bing.com/search?q=machine+learning"));
        }
        _ => panic!("Expected Bing Search target"),
    }

    // 3. DuckDuckGo bang (!ddg)
    let target_ddg = router.resolve("!ddg zero knowledge proofs");
    match target_ddg {
        norva::search::query_router::ResolvedTarget::Search { engine_name, query, target_url } => {
            assert_eq!(engine_name, "DuckDuckGo");
            assert_eq!(query, "zero knowledge proofs");
            assert!(target_url.contains("duckduckgo.com/?q=zero+knowledge+proofs"));
        }
        _ => panic!("Expected DuckDuckGo target"),
    }

    // 4. Yahoo bang (!y)
    let target_y = router.resolve("!y open source software");
    match target_y {
        norva::search::query_router::ResolvedTarget::Search { engine_name, query, target_url } => {
            assert_eq!(engine_name, "Yahoo Search");
            assert_eq!(query, "open source software");
            assert!(target_url.contains("search.yahoo.com/search?p=open+source+software"));
        }
        _ => panic!("Expected Yahoo target"),
    }

    // 5. Ahmia (.onion Tor Search) bang (!ahmia / !onion)
    let target_ahmia = router.resolve("!ahmia secure mail directory");
    match target_ahmia {
        norva::search::query_router::ResolvedTarget::Search { engine_name, query, target_url } => {
            assert_eq!(engine_name, "Ahmia (Tor .onion Search)");
            assert_eq!(query, "secure mail directory");
            assert!(target_url.contains("ahmia.fi/search/?q=secure+mail+directory"));
        }
        _ => panic!("Expected Ahmia target"),
    }

    let target_onion = router.resolve("!onion hidden services");
    match target_onion {
        norva::search::query_router::ResolvedTarget::Search { engine_name, query, target_url } => {
            assert_eq!(engine_name, "Ahmia (Tor .onion Search)");
            assert_eq!(query, "hidden services");
            assert!(target_url.contains("ahmia.fi/search/?q=hidden+services"));
        }
        _ => panic!("Expected Ahmia target via !onion alias"),
    }

    // 6. Direct URL navigation
    let target_url = router.resolve("https://example.com");
    assert_eq!(
        target_url,
        norva::search::query_router::ResolvedTarget::DirectUrl("https://example.com".to_string())
    );

    let target_bare_domain = router.resolve("rust-lang.org");
    assert_eq!(
        target_bare_domain,
        norva::search::query_router::ResolvedTarget::DirectUrl("https://rust-lang.org".to_string())
    );

    // 7. Internal NORVA pages
    let target_internal = router.resolve("norva://shields");
    assert_eq!(
        target_internal,
        norva::search::query_router::ResolvedTarget::InternalPage("norva://shields".to_string())
    );

    // 8. Plain search without bangs (routes to default DuckDuckGo)
    let target_plain = router.resolve("rust concurrency patterns");
    match target_plain {
        norva::search::query_router::ResolvedTarget::Search { engine_name, query, target_url } => {
            assert_eq!(engine_name, "DuckDuckGo");
            assert_eq!(query, "rust concurrency patterns");
            assert!(target_url.contains("duckduckgo.com/?q=rust+concurrency+patterns"));
        }
        _ => panic!("Expected default engine search target"),
    }
}

#[test]
fn test_gtk_chrome_css_validity() {
    if gtk::init().is_err() {
        return;
    }
    let _ = norva::ui::NorvaGtkChrome::new(false);
}
