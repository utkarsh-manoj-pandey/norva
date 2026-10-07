// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/browser/app.rs
// Description: Main browser lifecycle controller, window constructor & WebKit orchestrator.
// ============================================================================

use std::borrow::Cow;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{
    http::{header, Response},
    WebContext, WebViewBuilder,
};

use crate::browser::ipc::{IpcDispatcher, NorvaUserEvent};
use crate::browser::state::BrowserState;
use crate::config::NorvaConfig;
use crate::db::DatabaseManager;
use crate::privacy::{AdBlockEngine, CacheManager, CookieManager, FilterAction, PermissionsManager};
use crate::search::{QueryRouter, SearchEngine};
use crate::ui::{
    render_cookies_page, render_history_page, render_newtab_page,
    render_permissions_page, render_settings_page, render_shields_page,
};

/// Master application runtime for the NORVA Browser.
///
/// I am writing this struct because building a truly private, robust, and modern browser
/// requires harmonious coordination between the operating system's window manager (Tao),
/// the WebKit layout engine (Wry), our 5 isolated SQLite databases, and our privacy shields.
pub struct NorvaBrowser {
    config: NorvaConfig,
    initial_url: Option<String>,
}

impl NorvaBrowser {
    /// Constructs a new NORVA browser instance with the given configuration and optional initial URL.
    pub fn new(config: NorvaConfig, initial_url: Option<String>) -> Self {
        Self {
            config,
            initial_url,
        }
    }

    /// Launches the browser window and enters the Tao event loop.
    ///
    /// I have written this execution pipeline to carefully sequence each subsystem:
    /// 1. Initialize all 5 SQLite databases with WAL mode and strict permissions.
    /// 2. Initialize the Adblock and Anti-tracking engine with pre-compiled surveillance blocklists.
    /// 3. Initialize hardware permissions arbiter for Internal/External Cameras, Microphones, and GPS.
    /// 4. Register the custom `norva://` protocol to serve lightning-fast internal UI pages directly from memory.
    /// 5. Hook the navigation handler to pre-emptively block tracker sockets and strip tracking URL parameters.
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let is_secure = self.config.secure_mode;

        // 1. Initialize Multi-Database Architecture (5 separate SQLite databases)
        let db = DatabaseManager::init(&self.config)?;

        // 2. Initialize Privacy Subsystems
        let adblock = AdBlockEngine::new(db.shields.clone());
        let permissions_mgr = PermissionsManager::new(db.permissions.clone());
        let _cookie_mgr = CookieManager::new(db.cache_cookies.clone());
        let cache_mgr = CacheManager::new(db.cache_cookies.clone(), self.config.web_cache_dir.clone());
        let query_router = QueryRouter::new(db.settings.clone());
        let state = BrowserState::new(is_secure);

        // 3. Initialize Tao Event Loop with Custom User Event Support
        let mut event_loop_builder = EventLoopBuilder::<NorvaUserEvent>::with_user_event();
        let event_loop = event_loop_builder.build();
        let proxy = event_loop.create_proxy();

        // 4. Construct the Main Native Browser Window
        let title = if is_secure {
            "NORVA — Private Ghost Session"
        } else {
            "NORVA"
        };

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(1280.0, 840.0))
            .with_min_inner_size(LogicalSize::new(800.0, 600.0))
            .build(&event_loop)?;

        // 5. Setup WebKit Context (Isolated storage directory in normal mode; ephemeral in secure mode)
        let mut web_context = if is_secure {
            // In Secure Mode, no persistent WebKit context directory is allocated
            WebContext::new(None)
        } else {
            WebContext::new(Some(self.config.web_data_dir.clone()))
        };

        // 6. Setup IPC Dispatcher
        let ipc_dispatcher = IpcDispatcher::new(
            proxy.clone(),
            db.clone(),
            query_router.clone(),
            cache_mgr.clone(),
        );

        // Clones for closures
        let db_proto = db.clone();
        let cache_proto = cache_mgr.clone();
        let adblock_nav = adblock.clone();
        let db_nav = db.clone();
        let state_nav = state.clone();
        let proxy_nav = proxy.clone();
        let perm_mgr = permissions_mgr.clone();
        let state_perm = state.clone();

        // Combined cosmetic CSS + anti-fingerprint protection
        let cosmetic_css = adblock.get_cosmetic_filter_css();
        let anti_fp_js = adblock.get_anti_fingerprinting_js();
        let init_script = format!(
            r#"
            (function() {{
                try {{
                    const style = document.createElement('style');
                    style.textContent = `{}`;
                    if (document.head) {{
                        document.head.appendChild(style);
                    }} else {{
                        document.addEventListener('DOMContentLoaded', () => document.head.appendChild(style));
                    }}
                }} catch(e) {{}}
            }})();
            {}
            "#,
            cosmetic_css, anti_fp_js
        );

        // 7. Assemble the WebView Builder with all security layers
        let builder = WebViewBuilder::new_with_web_context(&mut web_context)
            .with_incognito(is_secure)
            .with_initialization_script(&init_script)
            .with_custom_protocol("norva".to_string(), move |_id, req| {
                // I am writing this protocol handler because serving internal dashboard pages
                // directly through memory guarantees instantaneous load times without any local HTTP server port exposure.
                let uri = req.uri().to_string();
                let path = if uri.contains("://") {
                    uri.split("://").nth(1).unwrap_or("newtab").trim_end_matches('/')
                } else {
                    "newtab"
                };

                let html = match path {
                    "newtab" | "" => {
                        let stats = db_proto.shields.get_stats().unwrap_or(crate::db::shields_db::ShieldStats {
                            total_ads_blocked: 0,
                            total_trackers_blocked: 0,
                            total_fingerprinters_blocked: 0,
                            total_params_stripped: 0,
                            total_all_time: 0,
                            estimated_bandwidth_saved_kb: 0,
                        });
                        let engines = db_proto.settings.get_all_search_engines().unwrap_or_default();
                        let default_eng = db_proto.settings.get_default_engine();
                        let cache_mb = (cache_proto.get_cache_size_bytes() as f64) / (1024.0 * 1024.0);
                        render_newtab_page(&stats, &engines, &default_eng, is_secure, cache_mb)
                    }
                    "history" => {
                        let entries = db_proto.history.get_recent(200).unwrap_or_default();
                        render_history_page(&entries)
                    }
                    "shields" => {
                        let stats = db_proto.shields.get_stats().unwrap_or(crate::db::shields_db::ShieldStats {
                            total_ads_blocked: 0,
                            total_trackers_blocked: 0,
                            total_fingerprinters_blocked: 0,
                            total_params_stripped: 0,
                            total_all_time: 0,
                            estimated_bandwidth_saved_kb: 0,
                        });
                        let events = db_proto.shields.get_recent_blocks(100).unwrap_or_default();
                        render_shields_page(&stats, &events)
                    }
                    "permissions" => {
                        let perms = db_proto.permissions.get_all_permissions().unwrap_or_default();
                        let audits = db_proto.permissions.get_recent_audits(100).unwrap_or_default();
                        render_permissions_page(&perms, &audits)
                    }
                    "cookies" => {
                        let cache_mb = (cache_proto.get_cache_size_bytes() as f64) / (1024.0 * 1024.0);
                        render_cookies_page(cache_mb)
                    }
                    "settings" => {
                        let engines = db_proto.settings.get_all_search_engines().unwrap_or_default();
                        let default_eng = db_proto.settings.get_default_engine();
                        render_settings_page(&engines, &default_eng, is_secure)
                    }
                    _ => {
                        let stats = db_proto.shields.get_stats().unwrap_or_default();
                        let engines = db_proto.settings.get_all_search_engines().unwrap_or_default();
                        let default_eng = db_proto.settings.get_default_engine();
                        let cache_mb = (cache_proto.get_cache_size_bytes() as f64) / (1024.0 * 1024.0);
                        render_newtab_page(&stats, &engines, &default_eng, is_secure, cache_mb)
                    }
                };

                Response::builder()
                    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                    .body(Cow::Owned(html.into_bytes()))
                    .unwrap()
            })
            .with_permission_handler(move |kind| {
                // I am writing this permission interception callback to enforce hardware access rules
                // across internal/external webcams, microphones, and geolocation sensors.
                let active_url = state_perm.get_url();
                perm_mgr.handle_permission_request(kind, Some(&active_url))
            })
            .with_navigation_handler(move |url| {
                // I am writing this navigation filter to enforce adblocking, tracker prevention,
                // and URL parameter sanitization before any external connection is established.
                match adblock_nav.evaluate_url(&url) {
                    FilterAction::Block { category, domain } => {
                        println!("[NORVA SHIELD] Blocked {:?} targeting: {}", category, domain);
                        let count = state_nav.inc_blocked();
                        let _ = proxy_nav.send_event(NorvaUserEvent::UpdateShieldCount(count));
                        false
                    }
                    FilterAction::SanitizeUrl(clean_url) => {
                        println!("[NORVA SHIELD] Stripped surveillance parameters -> {}", clean_url);
                        let count = state_nav.inc_blocked();
                        let _ = proxy_nav.send_event(NorvaUserEvent::UpdateShieldCount(count));
                        let _ = proxy_nav.send_event(NorvaUserEvent::Navigate(clean_url));
                        false
                    }
                    FilterAction::Allow => {
                        // Update active state
                        state_nav.set_url(&url);

                        // If user performed a search on Google, Bing, DDG, Yahoo, or Ahmia, record it
                        if let Some((query, engine)) = SearchEngine::detect_search(&url) {
                            let _ = db_nav.history.record_visit(&url, &engine, Some(&query), Some(&engine));
                        } else {
                            let _ = db_nav.history.record_visit(&url, &url, None, None);
                        }

                        true
                    }
                }
            })
            .with_ipc_handler(move |req| {
                ipc_dispatcher.handle_raw_message(req.body());
            });

        // 8. Wire document title changed and page load handlers for live UI synchronization
        let proxy_title = proxy.clone();
        let proxy_load = proxy.clone();
        let builder = builder
            .with_document_title_changed_handler(move |title| {
                let _ = proxy_title.send_event(NorvaUserEvent::PageTitleChanged(title));
            })
            .with_on_page_load_handler(move |_event, url| {
                let _ = proxy_load.send_event(NorvaUserEvent::PageUrlChanged(url));
            });

        let start_url = self.initial_url.unwrap_or_else(|| "norva://newtab".to_string());

        // 9. Attach the authentic native GTK Chrome (Tabs, Omnibox, Toolbar, Bookmarks) and WebKit Viewport
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        let (webview, chrome) = {
            use gtk::prelude::*;
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;

            let vbox = window
                .default_vbox()
                .ok_or_else(|| "Failed to obtain GTK default vbox from Tao window")?;

            let chrome = std::rc::Rc::new(crate::ui::NorvaGtkChrome::new(is_secure));

            // Pack the authentic browser chrome header at the top
            vbox.pack_start(&chrome.root, false, false, 0);

            // Pack a dedicated webview box to host the WebKitGTK container
            let webview_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
            vbox.pack_start(&webview_box, true, true, 0);

            let webview = builder
                .with_url(&start_url)
                .build_gtk(&webview_box)?;

            // 1. Initial Tab (Tab 1)
            let proxy_switch = proxy.clone();
            let proxy_close = proxy.clone();
            chrome.add_tab(
                1,
                "NORVA Start",
                &start_url,
                "🛡️",
                move |id| {
                    let _ = proxy_switch.send_event(NorvaUserEvent::SwitchTab(id));
                },
                move |id| {
                    let _ = proxy_close.send_event(NorvaUserEvent::CloseTab(id));
                },
            );

            // 2. Wire Navigation buttons (Back, Forward, Reload, Home)
            let proxy_back = proxy.clone();
            chrome.back_btn.connect_clicked(move |_| {
                let _ = proxy_back.send_event(NorvaUserEvent::GoBack);
            });

            let proxy_fwd = proxy.clone();
            chrome.fwd_btn.connect_clicked(move |_| {
                let _ = proxy_fwd.send_event(NorvaUserEvent::GoForward);
            });

            let proxy_reload = proxy.clone();
            chrome.reload_btn.connect_clicked(move |_| {
                let _ = proxy_reload.send_event(NorvaUserEvent::Reload);
            });

            let proxy_home = proxy.clone();
            chrome.home_btn.connect_clicked(move |_| {
                let _ = proxy_home.send_event(NorvaUserEvent::Navigate("norva://newtab".to_string()));
            });

            let proxy_nt = proxy.clone();
            chrome.new_tab_btn.connect_clicked(move |_| {
                let _ = proxy_nt.send_event(NorvaUserEvent::NewTab);
            });

            // 3. Wire Omnibox Enter Activation
            let omnibox_clone = chrome.omnibox.clone();
            let qr_clone = query_router.clone();
            let proxy_omni = proxy.clone();
            chrome.omnibox.connect_activate(move |_| {
                let input = omnibox_clone.text().to_string();
                let trimmed = input.trim();
                if !trimmed.is_empty() {
                    let resolved = qr_clone.resolve(trimmed);
                    let _ = proxy_omni.send_event(NorvaUserEvent::Navigate(resolved.to_url()));
                }
            });

            // 4. Wire Bookmarks Bar
            let proxy_bm = proxy.clone();
            chrome.populate_bookmarks(move |url| {
                let _ = proxy_bm.send_event(NorvaUserEvent::Navigate(url.to_string()));
            });

            // 5. Wire Bookmark star button (★)
            let b_box = chrome.bookmarks_box.clone();
            let b_omni = chrome.omnibox.clone();
            let proxy_star = proxy.clone();
            chrome.bookmark_btn.connect_clicked(move |_| {
                let url = b_omni.text().to_string();
                if !url.is_empty() {
                    let label = if url.starts_with("norva://") {
                        format!("🛡️ {}", url.trim_start_matches("norva://"))
                    } else if let Some(domain) = url.split("://").nth(1).and_then(|d| d.split('/').next()) {
                        format!("⭐ {}", domain)
                    } else {
                        "⭐ Bookmark".to_string()
                    };
                    let b_btn = gtk::Button::with_label(&label);
                    b_btn.style_context().add_class("norva-bookmark-pill");
                    b_btn.set_tooltip_text(Some(&url));
                    let nav_p = proxy_star.clone();
                    let target_url = url.clone();
                    b_btn.connect_clicked(move |_| {
                        let _ = nav_p.send_event(NorvaUserEvent::Navigate(target_url.clone()));
                    });
                    b_btn.show_all();
                    b_box.pack_start(&b_btn, false, false, 0);
                }
            });

            // 6. Wire Engine Dropdown Popover
            let proxy_eng = proxy.clone();
            chrome.attach_engine_popover(move |id, name| {
                let _ = proxy_eng.send_event(NorvaUserEvent::SelectEngine {
                    id: id.to_string(),
                    name: name.to_string(),
                });
            });

            // 7. Wire Shields Popover
            let proxy_shield = proxy.clone();
            chrome.attach_shield_popover(move || {
                let _ = proxy_shield.send_event(NorvaUserEvent::Navigate("norva://shields".to_string()));
            });

            // 8. Wire Hardware Popovers
            let proxy_hw = proxy.clone();
            chrome.attach_hardware_popovers(move || {
                let _ = proxy_hw.send_event(NorvaUserEvent::Navigate("norva://permissions".to_string()));
            });

            // 9. Wire Security Popover
            let proxy_sec = proxy.clone();
            chrome.attach_security_popover(move || {
                let _ = proxy_sec.send_event(NorvaUserEvent::Navigate("norva://permissions".to_string()));
            });

            // 10. Wire 3-dots Menu Popover
            let proxy_menu_nav = proxy.clone();
            let proxy_menu_nt = proxy.clone();
            chrome.attach_menu_popover(
                move |url| {
                    let _ = proxy_menu_nav.send_event(NorvaUserEvent::Navigate(url.to_string()));
                },
                move || {
                    let _ = proxy_menu_nt.send_event(NorvaUserEvent::NewTab);
                },
            );

            // Ensure all GTK chrome widgets are visible and interactive
            vbox.show_all();
            window.gtk_window().present();

            (webview, chrome)
        };

        #[cfg(not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        let webview = builder
            .with_url(&start_url)
            .build(&window)?;

        // 10. Run Tao Event Loop
        let db_event = db.clone();
        let proxy_tab = proxy.clone();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::UserEvent(user_event) => match user_event {
                    NorvaUserEvent::Navigate(url) => {
                        let _ = webview.load_url(&url);
                        #[cfg(target_os = "linux")]
                        {
                            let active_id = *chrome.active_tab_id.borrow();
                            let current_title = chrome.get_tab_title(active_id).unwrap_or_else(|| "Loading...".to_string());
                            let icon = get_url_icon(&url);
                            chrome.update_active_tab(&current_title, &url, icon);
                        }
                    }
                    NorvaUserEvent::GoBack => {
                        let _ = webview.go_back();
                    }
                    NorvaUserEvent::GoForward => {
                        let _ = webview.go_forward();
                    }
                    NorvaUserEvent::Reload => {
                        let _ = webview.reload();
                    }
                    NorvaUserEvent::EvalScript(script) => {
                        let _ = webview.evaluate_script(&script);
                    }
                    NorvaUserEvent::PageTitleChanged(title) => {
                        #[cfg(target_os = "linux")]
                        {
                            let active_id = *chrome.active_tab_id.borrow();
                            let current_url = chrome.get_tab_url(active_id).unwrap_or_default();
                            let icon = get_url_icon(&current_url);
                            chrome.update_active_tab(&title, &current_url, icon);
                        }
                    }
                    NorvaUserEvent::PageUrlChanged(url) => {
                        #[cfg(target_os = "linux")]
                        {
                            let active_id = *chrome.active_tab_id.borrow();
                            let title = chrome.get_tab_title(active_id).unwrap_or_else(|| "NORVA".to_string());
                            let icon = get_url_icon(&url);
                            chrome.update_active_tab(&title, &url, icon);
                        }
                    }
                    NorvaUserEvent::UpdateShieldCount(count) => {
                        #[cfg(target_os = "linux")]
                        {
                            chrome.set_shield_count(count);
                        }
                    }
                    NorvaUserEvent::NewTab => {
                        #[cfg(target_os = "linux")]
                        {
                            let new_id = chrome.get_next_tab_id();
                            let p_sw = proxy_tab.clone();
                            let p_cl = proxy_tab.clone();
                            chrome.add_tab(
                                new_id,
                                "New Tab",
                                "norva://newtab",
                                "🌐",
                                move |id| {
                                    let _ = p_sw.send_event(NorvaUserEvent::SwitchTab(id));
                                },
                                move |id| {
                                    let _ = p_cl.send_event(NorvaUserEvent::CloseTab(id));
                                },
                            );
                            let _ = webview.load_url("norva://newtab");
                        }
                    }
                    NorvaUserEvent::SwitchTab(id) => {
                        #[cfg(target_os = "linux")]
                        {
                            chrome.set_active_tab(id);
                            if let Some(url) = chrome.get_tab_url(id) {
                                let _ = webview.load_url(&url);
                            }
                        }
                    }
                    NorvaUserEvent::CloseTab(id) => {
                        #[cfg(target_os = "linux")]
                        {
                            let next = chrome.remove_tab(id);
                            if let Some(next_id) = next {
                                chrome.set_active_tab(next_id);
                                if let Some(url) = chrome.get_tab_url(next_id) {
                                    let _ = webview.load_url(&url);
                                }
                            } else {
                                // All tabs closed: open fresh new tab
                                let new_id = chrome.get_next_tab_id();
                                let p_sw = proxy_tab.clone();
                                let p_cl = proxy_tab.clone();
                                chrome.add_tab(
                                    new_id,
                                    "New Tab",
                                    "norva://newtab",
                                    "🌐",
                                    move |id| {
                                        let _ = p_sw.send_event(NorvaUserEvent::SwitchTab(id));
                                    },
                                    move |id| {
                                        let _ = p_cl.send_event(NorvaUserEvent::CloseTab(id));
                                    },
                                );
                                let _ = webview.load_url("norva://newtab");
                            }
                        }
                    }
                    NorvaUserEvent::SelectEngine { id, name: _ } => {
                        let _ = db_event.settings.set_default_engine(&id);
                    }
                },
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });
    }
}

/// Helper function to determine a suitable tab favicon icon based on destination URL.
fn get_url_icon(url: &str) -> &'static str {
    if url.starts_with("norva://") {
        "🛡️"
    } else if url.contains("google.") {
        "🔍"
    } else if url.contains("bing.") {
        "⚡"
    } else if url.contains("duckduckgo.") {
        "🦆"
    } else if url.contains("yahoo.") {
        "🟣"
    } else if url.contains("ahmia.") {
        "🧅"
    } else if url.contains("wikipedia.org") {
        "📚"
    } else if url.contains("rust-lang.org") {
        "🦀"
    } else if url.contains("github.com") {
        "🐙"
    } else {
        "🌐"
    }
}
