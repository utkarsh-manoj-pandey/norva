// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/ui/gtk_chrome.rs
// Description: Native GTK-3 Browser Header with authentic Tabs, Omnibox, Shields & Toolbars.
// ============================================================================

use gtk::prelude::*;
use gtk::{
    Box, Button, CssProvider, Entry, EventBox, Label, Orientation, Popover,
    StyleContext, STYLE_PROVIDER_PRIORITY_USER,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Represents a single browser tab in the native GTK tab bar.
#[derive(Clone)]
pub struct ChromeTab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub icon: String,
    pub widget: EventBox,
    pub title_label: Label,
    pub icon_label: Label,
}

/// The Native GTK Browser Chrome controller.
///
/// I am writing this native GTK chrome component because a web browser MUST have
/// an unmistakable, authentic desktop browser appearance:
/// 1. A dedicated Tab strip at the very top with interactive rounded tab pills and close buttons.
/// 2. A navigation toolbar with Back, Forward, Reload, Home, and a centered Omnibox.
/// 3. A live Privacy Shield badge, Hardware zero-trust indicators (Camera, Mic, GPS), and Menu.
/// 4. A bookmarks bar for instantaneous privacy access.
///
/// By rendering this in native GTK rather than an injected page script, the browser chrome
/// remains permanently fixed at the top regardless of what website the user visits.
#[derive(Clone)]
pub struct NorvaGtkChrome {
    pub root: Box,
    pub tabs_box: Box,
    pub omnibox: Entry,
    pub engine_label: Label,
    pub shield_btn: Button,
    pub security_btn: Button,
    pub bookmark_btn: Button,
    pub cam_btn: Button,
    pub mic_btn: Button,
    pub loc_btn: Button,
    pub back_btn: Button,
    pub fwd_btn: Button,
    pub reload_btn: Button,
    pub home_btn: Button,
    pub new_tab_btn: Button,
    pub menu_btn: Button,
    pub engine_btn: Button,
    pub bookmarks_box: Box,
    pub tabs: Rc<RefCell<Vec<ChromeTab>>>,
    pub active_tab_id: Rc<RefCell<usize>>,
    pub total_blocked: Rc<RefCell<usize>>,
    pub selected_engine: Rc<RefCell<String>>,
    pub next_tab_counter: Rc<RefCell<usize>>,
}

impl NorvaGtkChrome {
    /// Constructs and styles the complete native browser chrome.
    pub fn new(secure_mode: bool) -> Self {
        // Apply custom modern dark-mode CSS to the application screen
        Self::apply_theme_css();

        let root = Box::new(Orientation::Vertical, 0);
        root.style_context().add_class("norva-chrome-root");

        // --------------------------------------------------------------------
        // 1. TOP TAB BAR ROW (Firefox Proton style)
        // --------------------------------------------------------------------
        let tab_bar = Box::new(Orientation::Horizontal, 0);
        tab_bar.style_context().add_class("norva-tab-bar");

        // Container holding all open tabs directly on the left
        let tabs_box = Box::new(Orientation::Horizontal, 2);
        tabs_box.style_context().add_class("norva-tabs-container");
        tab_bar.pack_start(&tabs_box, false, false, 2);

        // New Tab button [+] right next to tabs
        let new_tab_btn = Button::with_label("+");
        new_tab_btn.style_context().add_class("norva-new-tab-btn");
        new_tab_btn.set_tooltip_text(Some("Open a new tab (Ctrl+T)"));
        tab_bar.pack_start(&new_tab_btn, false, false, 3);

        // Spacer pushing the mode badge to the right
        let spacer_tab = Box::new(Orientation::Horizontal, 0);
        tab_bar.pack_start(&spacer_tab, true, true, 0);

        // Mode badge on far right
        let mode_label = Label::new(None);
        if secure_mode {
            mode_label.set_text("🔒 GHOST MODE");
            mode_label.style_context().add_class("norva-badge-secure");
            mode_label.set_tooltip_text(Some("Secure Ghost Mode: in-memory volatile session, zero disk writes"));
        } else {
            mode_label.set_text("● PRIVACY HARDENED");
            mode_label.style_context().add_class("norva-badge-normal");
            mode_label.set_tooltip_text(Some("Hardened Privacy Session: isolated SQLite databases, active tracking shields"));
        }
        tab_bar.pack_end(&mode_label, false, false, 8);

        root.pack_start(&tab_bar, false, false, 0);

        // --------------------------------------------------------------------
        // 2. MAIN NAVIGATION & OMNIBOX TOOLBAR
        // --------------------------------------------------------------------
        let toolbar = Box::new(Orientation::Horizontal, 4);
        toolbar.style_context().add_class("norva-toolbar");

        // Back, Forward, Reload, Home
        let back_btn = Button::with_label("←");
        back_btn.style_context().add_class("norva-nav-btn");
        back_btn.set_tooltip_text(Some("Click to go back"));

        let fwd_btn = Button::with_label("→");
        fwd_btn.style_context().add_class("norva-nav-btn");
        fwd_btn.set_tooltip_text(Some("Click to go forward"));

        let reload_btn = Button::with_label("↻");
        reload_btn.style_context().add_class("norva-nav-btn");
        reload_btn.set_tooltip_text(Some("Reload current page"));

        let home_btn = Button::with_label("⌂");
        home_btn.style_context().add_class("norva-nav-btn");
        home_btn.set_tooltip_text(Some("NORVA Start Page (norva://newtab)"));

        toolbar.pack_start(&back_btn, false, false, 0);
        toolbar.pack_start(&fwd_btn, false, false, 0);
        toolbar.pack_start(&reload_btn, false, false, 0);
        toolbar.pack_start(&home_btn, false, false, 0);

        // Centered Omnibox frame
        let omnibox_frame = Box::new(Orientation::Horizontal, 4);
        omnibox_frame.style_context().add_class("norva-omnibox-frame");

        let security_btn = Button::with_label("🔒");
        security_btn.style_context().add_class("norva-btn-flat");
        security_btn.set_tooltip_text(Some("Site Security & Connection Status"));

        let engine_btn = Button::new();
        engine_btn.style_context().add_class("norva-engine-pill");
        engine_btn.set_tooltip_text(Some("Switch Search Engine"));
        let engine_label = Label::new(Some("🦆 DDG ▼"));
        engine_btn.add(&engine_label);

        let omnibox = Entry::new();
        omnibox.style_context().add_class("norva-omnibox-entry");
        omnibox.set_placeholder_text(Some("Search with DuckDuckGo or enter address"));
        omnibox.set_hexpand(true);

        let bookmark_btn = Button::with_label("☆");
        bookmark_btn.style_context().add_class("norva-bookmark-star");
        bookmark_btn.set_tooltip_text(Some("Bookmark this page"));

        omnibox_frame.pack_start(&security_btn, false, false, 0);
        omnibox_frame.pack_start(&engine_btn, false, false, 2);
        omnibox_frame.pack_start(&omnibox, true, true, 2);
        omnibox_frame.pack_end(&bookmark_btn, false, false, 2);

        toolbar.pack_start(&omnibox_frame, true, true, 4);

        // Privacy Shield button beside omnibox (clean modern badge with counter)
        let shield_btn = Button::with_label("🛡️ 0");
        shield_btn.style_context().add_class("norva-shield-btn");
        shield_btn.set_tooltip_text(Some("NORVA Privacy Shields: Trackers & Ads blocked"));
        toolbar.pack_start(&shield_btn, false, false, 2);

        // Hardware Indicators cluster (available for popovers)
        let hw_cluster = Box::new(Orientation::Horizontal, 2);
        hw_cluster.style_context().add_class("norva-hw-cluster");

        let cam_btn = Button::with_label("📷");
        cam_btn.style_context().add_class("norva-hw-btn");
        cam_btn.set_tooltip_text(Some("Camera: Zero-Trust Guard"));

        let mic_btn = Button::with_label("🎙️");
        mic_btn.style_context().add_class("norva-hw-btn");
        mic_btn.set_tooltip_text(Some("Microphone: Zero-Trust Guard"));

        let loc_btn = Button::with_label("📍");
        loc_btn.style_context().add_class("norva-hw-btn");
        loc_btn.set_tooltip_text(Some("Location: Protected"));

        hw_cluster.pack_start(&cam_btn, false, false, 0);
        hw_cluster.pack_start(&mic_btn, false, false, 0);
        hw_cluster.pack_start(&loc_btn, false, false, 0);

        // Modern 3-dots Menu button (⋮)
        let menu_btn = Button::with_label("⋮");
        menu_btn.style_context().add_class("norva-nav-btn");
        menu_btn.set_tooltip_text(Some("Application Menu"));
        toolbar.pack_end(&menu_btn, false, false, 2);

        root.pack_start(&toolbar, false, false, 0);

        // --------------------------------------------------------------------
        // 3. BOOKMARKS / SPEED BAR
        // --------------------------------------------------------------------
        let bookmarks_box = Box::new(Orientation::Horizontal, 4);
        bookmarks_box.style_context().add_class("norva-bookmarks-bar");
        root.pack_start(&bookmarks_box, false, false, 0);

        Self {
            root,
            tabs_box,
            omnibox,
            engine_label,
            shield_btn,
            security_btn,
            bookmark_btn,
            cam_btn,
            mic_btn,
            loc_btn,
            back_btn,
            fwd_btn,
            reload_btn,
            home_btn,
            new_tab_btn,
            menu_btn,
            engine_btn,
            bookmarks_box,
            tabs: Rc::new(RefCell::new(Vec::new())),
            active_tab_id: Rc::new(RefCell::new(0)),
            total_blocked: Rc::new(RefCell::new(0)),
            selected_engine: Rc::new(RefCell::new("duckduckgo".to_string())),
            next_tab_counter: Rc::new(RefCell::new(1)),
        }
    }

    /// Adds standard privacy bookmarks to the bookmarks bar.
    pub fn populate_bookmarks<F>(&self, on_navigate: F)
    where
        F: Fn(&str) + 'static + Clone,
    {
        let bookmarks = [
            ("🦆 DuckDuckGo", "https://duckduckgo.com"),
            ("🧅 Ahmia (.onion)", "https://ahmia.fi"),
            ("🗽 EFF Privacy", "https://www.eff.org"),
            ("🧅 Tor Project", "https://www.torproject.org"),
            ("📚 Wikipedia", "https://www.wikipedia.org"),
            ("🦀 Rust Lang", "https://www.rust-lang.org"),
            ("🛡️ Shields Center", "norva://shields"),
            ("📜 History Vault", "norva://history"),
        ];

        for (label, url) in bookmarks {
            let b_btn = Button::with_label(label);
            b_btn.style_context().add_class("norva-bookmark-pill");
            b_btn.set_tooltip_text(Some(url));

            let on_nav = on_navigate.clone();
            b_btn.connect_clicked(move |_| {
                on_nav(url);
            });

            self.bookmarks_box.pack_start(&b_btn, false, false, 0);
        }
    }

    /// Adds a new tab to the tab strip.
    pub fn add_tab<F>(&self, id: usize, title: &str, url: &str, icon: &str, on_switch: F, on_close: impl Fn(usize) + 'static)
    where
        F: Fn(usize) + 'static,
    {
        let tab_event_box = EventBox::new();
        tab_event_box.style_context().add_class("norva-tab");

        let tab_box = Box::new(Orientation::Horizontal, 6);
        tab_box.style_context().add_class("norva-tab-inner");

        let icon_label = Label::new(Some(icon));
        icon_label.style_context().add_class("norva-tab-icon");

        let title_label = Label::new(Some(title));
        title_label.style_context().add_class("norva-tab-title");
        title_label.set_hexpand(true);
        title_label.set_xalign(0.0);
        title_label.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let close_btn = Button::with_label("✕");
        close_btn.style_context().add_class("norva-tab-close");
        close_btn.set_tooltip_text(Some("Close tab (Ctrl+W)"));

        tab_box.pack_start(&icon_label, false, false, 0);
        tab_box.pack_start(&title_label, true, true, 0);
        tab_box.pack_end(&close_btn, false, false, 0);

        tab_event_box.add(&tab_box);

        // Click on tab to switch or middle-click to close
        let on_switch_rc = Rc::new(on_switch);
        let on_switch_clone = on_switch_rc.clone();
        let on_close_rc = Rc::new(on_close);
        let on_close_middle = on_close_rc.clone();
        let on_close_click = on_close_rc.clone();

        tab_event_box.connect_button_press_event(move |_, event| {
            if event.button() == 2 {
                on_close_middle(id);
                gtk::glib::Propagation::Stop
            } else if event.button() == 1 {
                on_switch_clone(id);
                gtk::glib::Propagation::Proceed
            } else {
                gtk::glib::Propagation::Proceed
            }
        });

        // Click close to remove tab
        close_btn.connect_clicked(move |_| {
            on_close_click(id);
        });

        tab_event_box.show_all();
        self.tabs_box.pack_start(&tab_event_box, false, false, 0);

        let chrome_tab = ChromeTab {
            id,
            title: title.to_string(),
            url: url.to_string(),
            icon: icon.to_string(),
            widget: tab_event_box,
            title_label,
            icon_label,
        };

        self.tabs.borrow_mut().push(chrome_tab);
        self.set_active_tab(id);
    }

    /// Marks a tab as active, updating the highlight and omnibox.
    pub fn set_active_tab(&self, id: usize) {
        *self.active_tab_id.borrow_mut() = id;
        for tab in self.tabs.borrow().iter() {
            if tab.id == id {
                tab.widget.style_context().add_class("active-tab");
                self.omnibox.set_text(&tab.url);
            } else {
                tab.widget.style_context().remove_class("active-tab");
            }
        }
    }

    /// Removes a tab from the strip by ID.
    pub fn remove_tab(&self, id: usize) -> Option<usize> {
        let mut tabs = self.tabs.borrow_mut();
        let pos = tabs.iter().position(|t| t.id == id)?;
        let tab = tabs.remove(pos);
        self.tabs_box.remove(&tab.widget);

        if tabs.is_empty() {
            None
        } else {
            let next_active = if pos < tabs.len() {
                tabs[pos].id
            } else {
                tabs[tabs.len() - 1].id
            };
            Some(next_active)
        }
    }

    /// Updates the title, URL, and icon of the active tab.
    pub fn update_active_tab(&self, title: &str, url: &str, icon: &str) {
        let active_id = *self.active_tab_id.borrow();
        for tab in self.tabs.borrow_mut().iter_mut() {
            if tab.id == active_id {
                tab.title = title.to_string();
                tab.url = url.to_string();
                tab.icon = icon.to_string();
                tab.title_label.set_text(title);
                tab.icon_label.set_text(icon);
                break;
            }
        }
        self.omnibox.set_text(url);
    }

    /// Returns the next available unique tab ID.
    pub fn get_next_tab_id(&self) -> usize {
        let mut c = self.next_tab_counter.borrow_mut();
        *c += 1;
        *c
    }

    /// Returns the title of the tab with the given id.
    pub fn get_tab_title(&self, id: usize) -> Option<String> {
        self.tabs.borrow().iter().find(|t| t.id == id).map(|t| t.title.clone())
    }

    /// Returns the stored URL of the tab with the given id.
    pub fn get_tab_url(&self, id: usize) -> Option<String> {
        self.tabs.borrow().iter().find(|t| t.id == id).map(|t| t.url.clone())
    }

    /// Returns the total number of currently open tabs.
    pub fn tab_count(&self) -> usize {
        self.tabs.borrow().len()
    }

    /// Updates the live shield blocked counter.
    pub fn set_shield_count(&self, count: usize) {
        *self.total_blocked.borrow_mut() = count;
        self.shield_btn.set_label(&format!("🛡️ {}", count));
    }

    /// Attaches the Site Security & Privacy Popover to the security button inside the omnibox.
    pub fn attach_security_popover<F>(&self, on_open_permissions: F)
    where
        F: Fn() + 'static + Clone,
    {
        let popover = Popover::new(Some(&self.security_btn));
        popover.style_context().add_class("norva-popover");

        let pbox = Box::new(Orientation::Vertical, 6);
        pbox.set_margin_top(10);
        pbox.set_margin_bottom(10);
        pbox.set_margin_start(12);
        pbox.set_margin_end(12);

        let title = Label::new(Some("🔒 Connection & Site Security"));
        title.style_context().add_class("norva-popover-title");
        pbox.pack_start(&title, false, false, 2);

        let stat1 = Label::new(Some("• Security Status: Active & Sandboxed"));
        stat1.style_context().add_class("norva-status-green");
        stat1.set_xalign(0.0);

        let stat2 = Label::new(Some("• Hardware Zero-Trust: Camera, Mic, GPS Gated"));
        stat2.set_xalign(0.0);

        let stat3 = Label::new(Some("• Tracking Protection: Enabled (Adblock + Shields)"));
        stat3.set_xalign(0.0);

        let stat4 = Label::new(Some("• Local Storage: Partitioned & Isolated"));
        stat4.set_xalign(0.0);

        pbox.pack_start(&stat1, false, false, 2);
        pbox.pack_start(&stat2, false, false, 2);
        pbox.pack_start(&stat3, false, false, 2);
        pbox.pack_start(&stat4, false, false, 2);

        let perm_btn = Button::with_label("Manage Site Permissions");
        perm_btn.style_context().add_class("norva-btn-primary");
        let pop_clone = popover.clone();
        let on_open = on_open_permissions.clone();
        perm_btn.connect_clicked(move |_| {
            pop_clone.popdown();
            on_open();
        });
        pbox.pack_start(&perm_btn, false, false, 6);

        popover.add(&pbox);
        pbox.show_all();

        self.security_btn.connect_clicked(move |_| {
            popover.popup();
        });
    }

    /// Attaches the Search Engine selector popover to the engine button.
    pub fn attach_engine_popover<F>(&self, on_select: F)
    where
        F: Fn(&str, &str) + 'static + Clone,
    {
        let popover = Popover::new(Some(&self.engine_btn));
        popover.style_context().add_class("norva-popover");

        let pbox = Box::new(Orientation::Vertical, 4);
        pbox.set_margin_top(8);
        pbox.set_margin_bottom(8);
        pbox.set_margin_start(8);
        pbox.set_margin_end(8);

        let title = Label::new(Some("Select Search Engine"));
        title.style_context().add_class("norva-popover-title");
        pbox.pack_start(&title, false, false, 4);

        let engines = [
            ("duckduckgo", "🦆 DuckDuckGo (Default)"),
            ("google", "🔍 Google Search"),
            ("bing", "⚡ Microsoft Bing"),
            ("yahoo", "🟣 Yahoo Search"),
            ("ahmia", "🧅 Ahmia (.onion Tor Search)"),
            ("startpage", "🛡️ Startpage"),
            ("brave", "🦁 Brave Search"),
        ];

        let engine_label_clone = self.engine_label.clone();
        let selected_engine_clone = self.selected_engine.clone();
        for (id, name) in engines {
            let btn = Button::with_label(name);
            btn.style_context().add_class("norva-popover-item");

            let pop_clone = popover.clone();
            let on_sel = on_select.clone();
            let lbl = engine_label_clone.clone();
            let sel_eng = selected_engine_clone.clone();

            btn.connect_clicked(move |_| {
                let short_name = match id {
                    "duckduckgo" => "🦆 DDG ▼",
                    "google" => "🔍 Google ▼",
                    "bing" => "⚡ Bing ▼",
                    "yahoo" => "🟣 Yahoo ▼",
                    "ahmia" => "🧅 Ahmia ▼",
                    "startpage" => "🛡️ Startpage ▼",
                    _ => "🌐 Search ▼",
                };
                lbl.set_text(short_name);
                *sel_eng.borrow_mut() = id.to_string();
                on_sel(id, name);
                pop_clone.popdown();
            });

            pbox.pack_start(&btn, false, false, 2);
        }

        popover.add(&pbox);
        pbox.show_all();

        self.engine_btn.connect_clicked(move |_| {
            popover.popup();
        });
    }

    /// Attaches the Privacy Shield dashboard popover to the shield button.
    pub fn attach_shield_popover<F>(&self, on_open_shields: F)
    where
        F: Fn() + 'static + Clone,
    {
        let popover = Popover::new(Some(&self.shield_btn));
        popover.style_context().add_class("norva-popover");

        let pbox = Box::new(Orientation::Vertical, 6);
        pbox.set_margin_top(10);
        pbox.set_margin_bottom(10);
        pbox.set_margin_start(12);
        pbox.set_margin_end(12);

        let title = Label::new(Some("🛡️ NORVA Privacy Shields"));
        title.style_context().add_class("norva-popover-title");
        pbox.pack_start(&title, false, false, 2);

        let status_lbl = Label::new(Some("Status: All Shields UP & Enforced"));
        status_lbl.style_context().add_class("norva-status-green");
        pbox.pack_start(&status_lbl, false, false, 2);

        let item1 = Label::new(Some("• Advertising Network Sockets: BLOCKED"));
        item1.set_xalign(0.0);
        let item2 = Label::new(Some("• Analytics & Telemetry Beacons: BLOCKED"));
        item2.set_xalign(0.0);
        let item3 = Label::new(Some("• URL Click Tracking Tokens (fbclid, etc.): STRIPPED"));
        item3.set_xalign(0.0);
        let item4 = Label::new(Some("• Canvas & Audio Fingerprinting: RANDOMIZED"));
        item4.set_xalign(0.0);

        pbox.pack_start(&item1, false, false, 1);
        pbox.pack_start(&item2, false, false, 1);
        pbox.pack_start(&item3, false, false, 1);
        pbox.pack_start(&item4, false, false, 1);

        let open_btn = Button::with_label("Open Shields Command Center");
        open_btn.style_context().add_class("norva-btn-primary");
        let pop_clone = popover.clone();
        let on_open = on_open_shields.clone();
        open_btn.connect_clicked(move |_| {
            on_open();
            pop_clone.popdown();
        });
        pbox.pack_start(&open_btn, false, false, 6);

        popover.add(&pbox);
        pbox.show_all();

        self.shield_btn.connect_clicked(move |_| {
            popover.popup();
        });
    }

    /// Attaches the Browser Menu popover to the three-dots button.
    pub fn attach_menu_popover<F, N>(&self, on_navigate: F, on_new_tab: N)
    where
        F: Fn(&str) + 'static + Clone,
        N: Fn() + 'static + Clone,
    {
        let popover = Popover::new(Some(&self.menu_btn));
        popover.style_context().add_class("norva-popover");

        let pbox = Box::new(Orientation::Vertical, 4);
        pbox.set_margin_top(8);
        pbox.set_margin_bottom(8);
        pbox.set_margin_start(8);
        pbox.set_margin_end(8);

        let menu_items = [
            ("➕ New Tab", "new_tab"),
            ("📜 History Vault", "norva://history"),
            ("🛡️ Privacy Shields Center", "norva://shields"),
            ("📷 Site Permissions & Hardware", "norva://permissions"),
            ("🍪 Cookie Isolation & Cache Purge", "norva://cookies"),
            ("⚙️ Browser Settings", "norva://settings"),
            ("🏠 Start Page", "norva://newtab"),
        ];

        for (label, action) in menu_items {
            let btn = Button::with_label(label);
            btn.style_context().add_class("norva-popover-item");

            let pop_clone = popover.clone();
            let on_nav = on_navigate.clone();
            let on_nt = on_new_tab.clone();

            btn.connect_clicked(move |_| {
                pop_clone.popdown();
                if action == "new_tab" {
                    on_nt();
                } else {
                    on_nav(action);
                }
            });

            pbox.pack_start(&btn, false, false, 2);
        }

        popover.add(&pbox);
        pbox.show_all();

        self.menu_btn.connect_clicked(move |_| {
            popover.popup();
        });
    }

    /// Attaches the Hardware (Camera, Mic, GPS) popovers to their respective indicator buttons.
    pub fn attach_hardware_popovers<F>(&self, on_open_permissions: F)
    where
        F: Fn() + 'static + Clone,
    {
        let hw_buttons = [
            (&self.cam_btn, "📷 Camera Privacy Guard", "Internal & External webcam requests are strictly gated by permissions.db."),
            (&self.mic_btn, "🎙️ Microphone Privacy Guard", "Internal & External microphone inputs are guarded against eavesdropping."),
            (&self.loc_btn, "📍 Location Privacy Guard", "Geolocation sensors are shielded against stealth triangulation."),
        ];

        for (btn, title, desc) in hw_buttons {
            let popover = Popover::new(Some(btn));
            popover.style_context().add_class("norva-popover");

            let pbox = Box::new(Orientation::Vertical, 4);
            pbox.set_margin_top(8);
            pbox.set_margin_bottom(8);
            pbox.set_margin_start(10);
            pbox.set_margin_end(10);

            let t_lbl = Label::new(Some(title));
            t_lbl.style_context().add_class("norva-popover-title");
            pbox.pack_start(&t_lbl, false, false, 2);

            let d_lbl = Label::new(Some(desc));
            d_lbl.set_line_wrap(true);
            d_lbl.set_max_width_chars(28);
            pbox.pack_start(&d_lbl, false, false, 4);

            let m_btn = Button::with_label("Manage Site Permissions");
            m_btn.style_context().add_class("norva-btn-primary");
            let pop_clone = popover.clone();
            let on_open = on_open_permissions.clone();
            m_btn.connect_clicked(move |_| {
                pop_clone.popdown();
                on_open();
            });
            pbox.pack_start(&m_btn, false, false, 4);

            popover.add(&pbox);
            pbox.show_all();

            let pop_popup = popover.clone();
            btn.connect_clicked(move |_| {
                pop_popup.popup();
            });
        }
    }

    /// Applies rich, modern browser-quality GTK CSS to the entire application window.
    ///
    /// This CSS is carefully designed to replicate the visual polish of Arc Browser,
    /// Firefox Proton, and modern Chromium — with proper sizing, spacing, depth,
    /// smooth transitions, and a refined dark color palette.
    fn apply_theme_css() {
        let css = r###"

        /* ================================================================
           NORVA — Premium Modern Browser Chrome (Arc / Firefox Proton / Brave)
           ================================================================ */

        /* ---- GLOBAL RESET & BASE THEME ---- */
        window, .background {
            background-color: #16171b;
            color: #ededf2;
        }

        * {
            -gtk-icon-shadow: none;
            text-shadow: none;
            transition-duration: 120ms;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        }

        /* Completely reset all buttons to eliminate native GTK box borders */
        button,
        button.flat,
        button:link,
        button:visited {
            background-color: transparent;
            background-image: none;
            border-color: transparent;
            border-style: none;
            border-width: 0;
            box-shadow: none;
            text-shadow: none;
            -gtk-icon-shadow: none;
            outline: none;
            -gtk-outline-radius: 0;
            padding: 0;
            margin: 0;
        }

        button:focus {
            outline: none;
            box-shadow: none;
        }

        entry {
            background-color: transparent;
            background-image: none;
            border-color: transparent;
            border-style: none;
            border-width: 0;
            box-shadow: none;
            outline: none;
        }

        entry:focus {
            box-shadow: none;
        }

        .norva-chrome-root {
            background-color: #16171b;
        }

        /* ================================================================
           1. TAB STRIP (Authentic Floating / Connected Tabs)
           ================================================================ */
        .norva-tab-bar {
            background-color: #16171b;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            padding: 4px 10px 0 10px;
            min-height: 38px;
        }

        .norva-tabs-container {
            border: none;
        }

        .norva-tab {
            background-color: transparent;
            border-radius: 8px 8px 0 0;
            min-width: 150px;
            min-height: 34px;
            margin-right: 1px;
            border-top: 2px solid transparent;
            border-right: 1px solid rgba(255, 255, 255, 0.07);
        }

        .norva-tab:hover {
            background-color: rgba(255, 255, 255, 0.05);
        }

        .norva-tab.active-tab {
            background-color: #212228;
            border-right: 1px solid transparent;
            border-top: 2px solid #5884ff;
        }

        .norva-tab-inner {
            padding: 6px 8px 6px 12px;
        }

        .norva-tab-icon {
            font-size: 13px;
            margin-right: 6px;
            opacity: 0.85;
        }

        .active-tab .norva-tab-icon {
            opacity: 1.0;
        }

        .norva-tab-title {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
            font-size: 12px;
            font-weight: 500;
            color: #92929e;
        }

        .active-tab .norva-tab-title {
            color: #ffffff;
            font-weight: 600;
        }

        .norva-tab-close {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #72727e;
            border-radius: 50%;
            padding: 0;
            font-size: 11px;
            font-weight: 700;
            min-width: 20px;
            min-height: 20px;
            margin-left: 4px;
            opacity: 0.4;
        }

        .norva-tab:hover .norva-tab-close,
        .active-tab .norva-tab-close {
            opacity: 0.85;
        }

        .norva-tab-close:hover {
            background-color: rgba(255, 255, 255, 0.16);
            color: #ffffff;
            opacity: 1.0;
        }

        .norva-new-tab-btn {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #8c8c9a;
            border-radius: 50%;
            min-width: 28px;
            min-height: 28px;
            font-size: 17px;
            font-weight: 400;
            margin-left: 3px;
            margin-top: 3px;
            margin-bottom: 3px;
        }

        .norva-new-tab-btn:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #ffffff;
        }

        .norva-badge-normal {
            background-color: rgba(80, 200, 120, 0.08);
            border: 1px solid rgba(80, 200, 120, 0.2);
            color: #50c878;
            padding: 3px 10px;
            border-radius: 12px;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 10.5px;
            font-weight: 600;
            letter-spacing: 0.4px;
            margin-top: 4px;
            margin-bottom: 4px;
        }

        .norva-badge-secure {
            background-color: rgba(255, 82, 82, 0.1);
            border: 1px solid rgba(255, 82, 82, 0.25);
            color: #ff6b6b;
            padding: 3px 10px;
            border-radius: 12px;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 10.5px;
            font-weight: 600;
            letter-spacing: 0.4px;
            margin-top: 4px;
            margin-bottom: 4px;
        }

        /* ================================================================
           2. MAIN TOOLBAR & OMNIBOX
           ================================================================ */
        .norva-toolbar {
            background-color: #212228;
            border-bottom: 1px solid rgba(255, 255, 255, 0.04);
            padding: 4px 10px;
            min-height: 42px;
        }

        .norva-nav-btn {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #9d9da8;
            border-radius: 50%;
            min-width: 32px;
            min-height: 32px;
            font-size: 15px;
            font-weight: 600;
            padding: 0;
            margin: 0 1px;
        }

        .norva-nav-btn:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #ffffff;
        }

        .norva-nav-btn:active {
            background-color: rgba(255, 255, 255, 0.14);
            color: #5884ff;
        }

        /* Omnibox Capsule Frame */
        .norva-omnibox-frame {
            background-color: #16171b;
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 20px;
            padding: 0 10px;
            min-height: 34px;
            margin: 0 6px;
        }

        .norva-omnibox-frame:focus-within {
            border-color: #5884ff;
            box-shadow: 0 0 0 2px rgba(88, 132, 255, 0.25);
            background-color: #18191f;
        }

        .norva-btn-flat {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #50c878;
            padding: 0 4px;
            font-size: 13px;
            border-radius: 50%;
            min-width: 26px;
            min-height: 26px;
        }

        .norva-btn-flat:hover {
            background-color: rgba(80, 200, 120, 0.12);
        }

        .norva-engine-pill {
            background-color: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.07);
            color: #b0b0bb;
            border-radius: 12px;
            padding: 2px 8px;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 11px;
            font-weight: 500;
            min-height: 24px;
            margin: 0 4px;
        }

        .norva-engine-pill:hover {
            background-color: rgba(255, 255, 255, 0.1);
            border-color: rgba(255, 255, 255, 0.14);
            color: #ffffff;
        }

        .norva-omnibox-entry {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #ededf2;
            font-family: 'Inter', -apple-system, 'Segoe UI', system-ui, sans-serif;
            font-size: 13px;
            padding: 4px 6px;
            caret-color: #5884ff;
        }

        .norva-bookmark-star {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #7e7e8c;
            border-radius: 50%;
            min-width: 26px;
            min-height: 26px;
            font-size: 15px;
            padding: 0;
        }

        .norva-bookmark-star:hover {
            color: #ffca3a;
            background-color: rgba(255, 202, 58, 0.12);
        }

        /* Privacy Shields Badge */
        .norva-shield-btn {
            background-color: rgba(80, 200, 120, 0.08);
            border: 1px solid rgba(80, 200, 120, 0.2);
            color: #50c878;
            border-radius: 14px;
            padding: 3px 12px;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 11.5px;
            font-weight: 600;
            min-height: 30px;
            margin: 0 2px;
        }

        .norva-shield-btn:hover {
            background-color: rgba(80, 200, 120, 0.16);
            border-color: rgba(80, 200, 120, 0.35);
        }

        /* Hardware Cluster */
        .norva-hw-cluster {
            background-color: rgba(255, 255, 255, 0.03);
            border-radius: 12px;
            padding: 2px 4px;
            margin: 0 3px;
        }

        .norva-hw-btn {
            background-color: transparent;
            border: none;
            box-shadow: none;
            font-size: 13px;
            padding: 0;
            opacity: 0.55;
            border-radius: 6px;
            min-width: 26px;
            min-height: 26px;
        }

        .norva-hw-btn:hover {
            opacity: 1.0;
            background-color: rgba(255, 255, 255, 0.08);
        }

        /* ================================================================
           3. BOOKMARKS BAR (Unified with Toolbar)
           ================================================================ */
        .norva-bookmarks-bar {
            background-color: #212228;
            border-bottom: 1px solid rgba(255, 255, 255, 0.04);
            padding: 2px 10px;
            min-height: 28px;
        }

        .norva-bookmark-pill {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #92929e;
            border-radius: 6px;
            padding: 3px 8px;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 11.5px;
            font-weight: 500;
            min-height: 24px;
            margin: 0 1px;
        }

        .norva-bookmark-pill:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #ededf2;
        }

        /* ================================================================
           4. POPOVERS (Floating Glass Cards)
           ================================================================ */
        .norva-popover {
            background-color: #24252d;
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 12px;
            color: #ededf2;
            padding: 6px;
            box-shadow: 0 12px 40px rgba(0, 0, 0, 0.6), 0 2px 8px rgba(0, 0, 0, 0.4);
        }

        .norva-popover-title {
            font-family: 'Inter', system-ui, sans-serif;
            font-weight: 700;
            font-size: 13px;
            color: #ffffff;
            margin-bottom: 6px;
            padding: 4px 8px 2px 8px;
        }

        .norva-popover-item {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #b0b0bc;
            border-radius: 8px;
            padding: 8px 12px;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 12.5px;
            font-weight: 500;
            min-height: 32px;
        }

        .norva-popover-item:hover {
            background-color: rgba(88, 132, 255, 0.14);
            color: #ffffff;
        }

        .norva-btn-primary {
            background-color: #5884ff;
            border: none;
            box-shadow: none;
            color: #ffffff;
            border-radius: 8px;
            padding: 8px 14px;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 12px;
            font-weight: 600;
            min-height: 32px;
        }

        .norva-btn-primary:hover {
            background-color: #4875f7;
        }

        .norva-btn-primary:active {
            background-color: #3b66e3;
        }

        .norva-status-green {
            color: #50c878;
            font-family: 'Inter', system-ui, sans-serif;
            font-size: 12px;
            font-weight: 600;
        }

        /* ================================================================
           5. SCROLLBAR
           ================================================================ */
        scrollbar {
            background-color: transparent;
        }
        scrollbar slider {
            background-color: rgba(255, 255, 255, 0.12);
            border-radius: 999px;
            min-width: 6px;
            min-height: 6px;
        }
        scrollbar slider:hover {
            background-color: rgba(255, 255, 255, 0.22);
        }

        "###;

        let provider = CssProvider::new();
        if let Err(err) = provider.load_from_data(css.as_bytes()) {
            eprintln!("[NORVA GTK CSS PARSE ERROR] {}", err);
        }

        if let Some(screen) = gtk::gdk::Screen::default() {
            StyleContext::add_provider_for_screen(&screen, &provider, STYLE_PROVIDER_PRIORITY_USER);
        }
    }
}
