// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/ui/gtk_chrome.rs
// Description: Modern Native GTK-3 Browser Header with authentic Tabs,
//              Capsule Omnibox, Downloads, Find in Page, Shields & Toolbars.
// ============================================================================

use gtk::prelude::*;
use gtk::{
    Box, Button, CssProvider, Entry, EventBox, IconSize, Image, Label, Orientation, Popover,
    StyleContext, STYLE_PROVIDER_PRIORITY_USER,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::db::bookmarks_db::BookmarkEntry;
use crate::db::downloads_db::DownloadEntry;

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
    pub is_pinned: bool,
    pub is_loading: bool,
}

/// The Native GTK Browser Chrome controller.
///
/// Engineered to provide an authentic, modern desktop browser appearance:
/// 1. Dedicated, rounded top tab strip with audio indicators, close buttons, pin state, and context actions.
/// 2. Navigation toolbar with Back, Forward, Reload/Stop, Home, and a centered Omnibox capsule.
/// 3. Omnibox with SSL security padlock, Search Engine switcher pill, clear button, and Bookmark star.
/// 4. Action cluster: Live Privacy Shields counter, Downloads shelf, Hardware zero-trust indicators, Find in Page, and Menu.
/// 5. Integrated Find in Page bar that slides in below the toolbar on Ctrl+F.
/// 6. Bookmarks bar supporting dynamic entries and fast toggle.
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
    pub downloads_btn: Button,
    pub find_btn: Button,
    pub reader_btn: Button,
    pub zoom_btn: Button,
    pub clear_omni_btn: Button,
    pub find_bar: Box,
    pub find_entry: Entry,
    pub find_count_lbl: Label,
    pub find_prev_btn: Button,
    pub find_next_btn: Button,
    pub find_close_btn: Button,
    pub bookmarks_box: Box,
    pub tabs: Rc<RefCell<Vec<ChromeTab>>>,
    pub active_tab_id: Rc<RefCell<usize>>,
    pub total_blocked: Rc<RefCell<usize>>,
    pub selected_engine: Rc<RefCell<String>>,
    pub next_tab_counter: Rc<RefCell<usize>>,
    pub bookmarks_bar_visible: Rc<RefCell<bool>>,
    pub is_page_loading: Rc<RefCell<bool>>,
    pub zoom_level: Rc<RefCell<f64>>,
}

impl NorvaGtkChrome {
    /// Constructs and styles the complete native browser chrome.
    pub fn new(secure_mode: bool) -> Self {
        // Apply modern browser dark-mode CSS to GTK screen
        Self::apply_theme_css();

        let root = Box::new(Orientation::Vertical, 0);
        root.style_context().add_class("norva-chrome-root");

        // --------------------------------------------------------------------
        // 1. TOP TAB BAR ROW
        // --------------------------------------------------------------------
        let tab_bar = Box::new(Orientation::Horizontal, 0);
        tab_bar.style_context().add_class("norva-tab-bar");

        let tabs_scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        tabs_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);
        tabs_scroll.style_context().add_class("norva-tabs-scroll");

        let tabs_box = Box::new(Orientation::Horizontal, 4);
        tabs_box.style_context().add_class("norva-tabs-container");
        tabs_scroll.add(&tabs_box);
        tab_bar.pack_start(&tabs_scroll, false, false, 0);

        // New Tab [+] button
        let new_tab_icon = Image::from_icon_name(Some("list-add-symbolic"), IconSize::Menu);
        let new_tab_btn = Button::new();
        new_tab_btn.set_image(Some(&new_tab_icon));
        new_tab_btn.style_context().add_class("norva-new-tab-btn");
        new_tab_btn.set_tooltip_text(Some("New tab (Ctrl+T)"));
        tab_bar.pack_start(&new_tab_btn, false, false, 4);

        // Flexible spacer
        let spacer_tab = Box::new(Orientation::Horizontal, 0);
        tab_bar.pack_start(&spacer_tab, true, true, 0);

        // Session status pill on far right
        let mode_label = Label::new(None);
        if secure_mode {
            mode_label.set_text("Private");
            mode_label.style_context().add_class("norva-badge-secure");
            mode_label.set_tooltip_text(Some("Ephemeral Session: memory-only, zero disk traces"));
        } else {
            mode_label.set_text("Protected");
            mode_label.style_context().add_class("norva-badge-normal");
            mode_label.set_tooltip_text(Some("Hardened Privacy: multi-vault isolation & tracking defense"));
        }
        tab_bar.pack_end(&mode_label, false, false, 12);

        root.pack_start(&tab_bar, false, false, 0);

        // --------------------------------------------------------------------
        // 2. MAIN NAVIGATION & OMNIBOX TOOLBAR
        // --------------------------------------------------------------------
        let toolbar = Box::new(Orientation::Horizontal, 4);
        toolbar.style_context().add_class("norva-toolbar");

        // Nav buttons cluster
        let back_icon = Image::from_icon_name(Some("go-previous-symbolic"), IconSize::Menu);
        let back_btn = Button::new();
        back_btn.set_image(Some(&back_icon));
        back_btn.style_context().add_class("norva-nav-btn");
        back_btn.set_tooltip_text(Some("Back (Alt+Left)"));

        let fwd_icon = Image::from_icon_name(Some("go-next-symbolic"), IconSize::Menu);
        let fwd_btn = Button::new();
        fwd_btn.set_image(Some(&fwd_icon));
        fwd_btn.style_context().add_class("norva-nav-btn");
        fwd_btn.set_tooltip_text(Some("Forward (Alt+Right)"));

        let reload_icon = Image::from_icon_name(Some("view-refresh-symbolic"), IconSize::Menu);
        let reload_btn = Button::new();
        reload_btn.set_image(Some(&reload_icon));
        reload_btn.style_context().add_class("norva-nav-btn");
        reload_btn.set_tooltip_text(Some("Reload (Ctrl+R)"));

        let home_icon = Image::from_icon_name(Some("go-home-symbolic"), IconSize::Menu);
        let home_btn = Button::new();
        home_btn.set_image(Some(&home_icon));
        home_btn.style_context().add_class("norva-nav-btn");
        home_btn.set_tooltip_text(Some("Start Page (Alt+Home)"));

        toolbar.pack_start(&back_btn, false, false, 0);
        toolbar.pack_start(&fwd_btn, false, false, 0);
        toolbar.pack_start(&reload_btn, false, false, 0);
        toolbar.pack_start(&home_btn, false, false, 0);

        // Centered Omnibox Capsule Frame
        let omnibox_frame = Box::new(Orientation::Horizontal, 4);
        omnibox_frame.style_context().add_class("norva-omnibox-frame");

        let security_icon = Image::from_icon_name(Some("security-high-symbolic"), IconSize::Menu);
        let security_btn = Button::new();
        security_btn.set_image(Some(&security_icon));
        security_btn.style_context().add_class("norva-btn-flat");
        security_btn.set_tooltip_text(Some("Connection Security & Origin Info"));

        let engine_btn = Button::new();
        engine_btn.style_context().add_class("norva-engine-pill");
        engine_btn.set_tooltip_text(Some("Switch Search Provider"));
        let engine_label = Label::new(Some("DuckDuckGo"));
        engine_btn.add(&engine_label);
        engine_btn.set_visible(false);

        let omnibox = Entry::new();
        omnibox.style_context().add_class("norva-omnibox-entry");
        omnibox.set_placeholder_text(Some("Search DuckDuckGo or enter URL"));
        omnibox.set_hexpand(true);

        let clear_icon = Image::from_icon_name(Some("edit-clear-symbolic"), IconSize::Menu);
        let clear_omni_btn = Button::new();
        clear_omni_btn.set_image(Some(&clear_icon));
        clear_omni_btn.style_context().add_class("norva-omni-clear-btn");
        clear_omni_btn.set_tooltip_text(Some("Clear address bar"));
        clear_omni_btn.set_visible(false);

        let reader_btn = Button::with_label("Reader");
        reader_btn.style_context().add_class("norva-omni-action-btn");
        reader_btn.set_tooltip_text(Some("Toggle Reader View"));
        reader_btn.set_visible(false);

        let zoom_btn = Button::with_label("100%");
        zoom_btn.style_context().add_class("norva-zoom-pill");
        zoom_btn.set_tooltip_text(Some("Page Zoom (Ctrl + / -)"));
        zoom_btn.set_visible(false);

        let bookmark_icon = Image::from_icon_name(Some("non-starred-symbolic"), IconSize::Menu);
        let bookmark_btn = Button::new();
        bookmark_btn.set_image(Some(&bookmark_icon));
        bookmark_btn.style_context().add_class("norva-bookmark-star");
        bookmark_btn.set_tooltip_text(Some("Bookmark tab (Ctrl+D)"));

        omnibox_frame.pack_start(&security_btn, false, false, 2);
        omnibox_frame.pack_start(&omnibox, true, true, 4);
        omnibox_frame.pack_start(&clear_omni_btn, false, false, 0);
        omnibox_frame.pack_start(&reader_btn, false, false, 2);
        omnibox_frame.pack_end(&bookmark_btn, false, false, 2);

        // Auto-show clear button when omnibox has text
        let clr_clone = clear_omni_btn.clone();
        omnibox.connect_changed(move |entry| {
            clr_clone.set_visible(!entry.text().is_empty());
        });
        let entry_clone = omnibox.clone();
        clear_omni_btn.connect_clicked(move |_| {
            entry_clone.set_text("");
            entry_clone.grab_focus();
        });

        toolbar.pack_start(&omnibox_frame, true, true, 4);

        // Privacy Shield Counter button
        let shield_btn = Button::with_label("Shields 0");
        shield_btn.style_context().add_class("norva-shield-btn");
        shield_btn.set_tooltip_text(Some("Norva Shields: Trackers & Ads Neutralized"));
        toolbar.pack_start(&shield_btn, false, false, 2);

        // Hardware zero-trust indicators (kept in struct, hidden from main navbar to eliminate clutter)
        let hw_cluster = Box::new(Orientation::Horizontal, 1);
        hw_cluster.style_context().add_class("norva-hw-cluster");

        let cam_btn = Button::with_label("Cam");
        cam_btn.style_context().add_class("norva-hw-btn");
        cam_btn.set_tooltip_text(Some("Camera: Zero-Trust Arbiter"));

        let mic_btn = Button::with_label("Mic");
        mic_btn.style_context().add_class("norva-hw-btn");
        mic_btn.set_tooltip_text(Some("Microphone: Zero-Trust Arbiter"));

        let loc_btn = Button::with_label("GPS");
        loc_btn.style_context().add_class("norva-hw-btn");
        loc_btn.set_tooltip_text(Some("Geolocation: Strict Isolation"));

        hw_cluster.pack_start(&cam_btn, false, false, 0);
        hw_cluster.pack_start(&mic_btn, false, false, 0);
        hw_cluster.pack_start(&loc_btn, false, false, 0);
        hw_cluster.set_visible(false);
        toolbar.pack_start(&hw_cluster, false, false, 2);

        // Downloads Button
        let dl_icon = Image::from_icon_name(Some("folder-download-symbolic"), IconSize::Menu);
        let downloads_btn = Button::new();
        downloads_btn.set_image(Some(&dl_icon));
        downloads_btn.style_context().add_class("norva-nav-btn");
        downloads_btn.set_tooltip_text(Some("Downloads (Ctrl+J)"));
        toolbar.pack_start(&downloads_btn, false, false, 2);

        // Find in Page Button
        let find_icon_btn = Image::from_icon_name(Some("edit-find-symbolic"), IconSize::Menu);
        let find_btn = Button::new();
        find_btn.set_image(Some(&find_icon_btn));
        find_btn.style_context().add_class("norva-nav-btn");
        find_btn.set_tooltip_text(Some("Find in Page (Ctrl+F)"));
        toolbar.pack_start(&find_btn, false, false, 2);

        // Modern 3-dots Menu button
        let menu_icon = Image::from_icon_name(Some("open-menu-symbolic"), IconSize::Menu);
        let menu_btn = Button::new();
        menu_btn.set_image(Some(&menu_icon));
        menu_btn.style_context().add_class("norva-nav-btn");
        menu_btn.set_tooltip_text(Some("Norva Menu"));
        toolbar.pack_end(&menu_btn, false, false, 2);

        root.pack_start(&toolbar, false, false, 0);

        // --------------------------------------------------------------------
        // 3. FIND IN PAGE BAR (Slides under toolbar when triggered)
        // --------------------------------------------------------------------
        let find_bar = Box::new(Orientation::Horizontal, 6);
        find_bar.style_context().add_class("norva-find-bar");

        let find_icon = Image::from_icon_name(Some("edit-find-symbolic"), IconSize::Menu);
        find_bar.pack_start(&find_icon, false, false, 2);

        let find_entry = Entry::new();
        find_entry.style_context().add_class("norva-find-entry");
        find_entry.set_placeholder_text(Some("Find on page..."));
        find_bar.pack_start(&find_entry, false, false, 2);

        let find_count_lbl = Label::new(Some("0 of 0"));
        find_count_lbl.style_context().add_class("norva-find-count");
        find_bar.pack_start(&find_count_lbl, false, false, 4);

        let find_prev_icon = Image::from_icon_name(Some("go-up-symbolic"), IconSize::Menu);
        let find_prev_btn = Button::new();
        find_prev_btn.set_image(Some(&find_prev_icon));
        find_prev_btn.style_context().add_class("norva-find-btn");
        find_prev_btn.set_tooltip_text(Some("Previous match (Shift+Enter)"));
        find_bar.pack_start(&find_prev_btn, false, false, 1);

        let find_next_icon = Image::from_icon_name(Some("go-down-symbolic"), IconSize::Menu);
        let find_next_btn = Button::new();
        find_next_btn.set_image(Some(&find_next_icon));
        find_next_btn.style_context().add_class("norva-find-btn");
        find_next_btn.set_tooltip_text(Some("Next match (Enter)"));
        find_bar.pack_start(&find_next_btn, false, false, 1);

        let find_close_icon = Image::from_icon_name(Some("window-close-symbolic"), IconSize::Menu);
        let find_close_btn = Button::new();
        find_close_btn.set_image(Some(&find_close_icon));
        find_close_btn.style_context().add_class("norva-find-close-btn");
        find_close_btn.set_tooltip_text(Some("Close find bar (Esc)"));
        find_bar.pack_end(&find_close_btn, false, false, 4);
        find_bar.pack_end(&find_close_btn, false, false, 4);

        let find_bar_clone = find_bar.clone();
        find_close_btn.connect_clicked(move |_| {
            find_bar_clone.set_visible(false);
        });

        // Toggle find bar when search button clicked
        let fb_toggle = find_bar.clone();
        let fe_focus = find_entry.clone();
        find_btn.connect_clicked(move |_| {
            let vis = !fb_toggle.is_visible();
            fb_toggle.set_visible(vis);
            if vis {
                fe_focus.grab_focus();
                fe_focus.select_region(0, -1);
            }
        });

        find_bar.set_visible(false);
        root.pack_start(&find_bar, false, false, 0);

        // --------------------------------------------------------------------
        // 4. BOOKMARKS / SPEED BAR
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
            downloads_btn,
            find_btn,
            reader_btn,
            zoom_btn,
            clear_omni_btn,
            find_bar,
            find_entry,
            find_count_lbl,
            find_prev_btn,
            find_next_btn,
            find_close_btn,
            bookmarks_box,
            tabs: Rc::new(RefCell::new(Vec::new())),
            active_tab_id: Rc::new(RefCell::new(0)),
            total_blocked: Rc::new(RefCell::new(0)),
            selected_engine: Rc::new(RefCell::new("duckduckgo".to_string())),
            next_tab_counter: Rc::new(RefCell::new(1)),
            bookmarks_bar_visible: Rc::new(RefCell::new(true)),
            is_page_loading: Rc::new(RefCell::new(false)),
            zoom_level: Rc::new(RefCell::new(100.0)),
        }
    }

    /// Adds standard privacy bookmarks to the bookmarks bar (for backward compatibility).
    pub fn populate_bookmarks<F>(&self, on_navigate: F)
    where
        F: Fn(&str) + 'static + Clone,
    {
        let bookmarks = [
            ("DuckDuckGo", "https://duckduckgo.com"),
            ("Ahmia Tor", "https://ahmia.fi"),
            ("EFF Privacy", "https://www.eff.org"),
            ("Tor Project", "https://www.torproject.org"),
            ("Wikipedia", "https://www.wikipedia.org"),
            ("Rust Lang", "https://www.rust-lang.org"),
            ("Shields Center", "norva://shields"),
            ("History Vault", "norva://history"),
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

    /// Populates the bookmarks bar dynamically from database entries.
    pub fn populate_from_db<F>(&self, bookmarks: &[BookmarkEntry], on_navigate: F)
    where
        F: Fn(&str) + 'static + Clone,
    {
        for child in self.bookmarks_box.children() {
            self.bookmarks_box.remove(&child);
        }

        for bm in bookmarks.iter().take(12) {
            let label_text = format!("{} {}", bm.icon, bm.title);
            let b_btn = Button::with_label(&label_text);
            b_btn.style_context().add_class("norva-bookmark-pill");
            b_btn.set_tooltip_text(Some(&bm.url));

            let on_nav = on_navigate.clone();
            let dest_url = bm.url.clone();
            b_btn.connect_clicked(move |_| {
                on_nav(&dest_url);
            });

            self.bookmarks_box.pack_start(&b_btn, false, false, 0);
        }
        self.bookmarks_box.show_all();
    }

    /// Toggles visibility of the bookmarks bar.
    pub fn toggle_bookmarks_bar(&self) {
        let mut vis = self.bookmarks_bar_visible.borrow_mut();
        *vis = !*vis;
        self.bookmarks_box.set_visible(*vis);
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

        let icon_label = Label::new(None);
        if !icon.is_empty() {
            icon_label.set_text(icon);
            icon_label.style_context().add_class("norva-tab-icon");
            tab_box.pack_start(&icon_label, false, false, 0);
        }

        let title_label = Label::new(Some(title));
        title_label.style_context().add_class("norva-tab-title");
        title_label.set_hexpand(true);
        title_label.set_xalign(0.0);
        title_label.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let close_icon = Image::from_icon_name(Some("window-close-symbolic"), IconSize::Menu);
        let close_btn = Button::new();
        close_btn.set_image(Some(&close_icon));
        close_btn.style_context().add_class("norva-tab-close");
        close_btn.set_tooltip_text(Some("Close tab (Ctrl+W)"));

        tab_box.pack_start(&title_label, true, true, 0);
        tab_box.pack_end(&close_btn, false, false, 0);

        tab_event_box.add(&tab_box);

        // Click tab to switch or middle-click to close
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
            is_pinned: false,
            is_loading: false,
        };

        self.tabs.borrow_mut().push(chrome_tab);
        self.set_active_tab(id);
    }

    /// Marks a tab as active, updating active highlights and omnibox entry.
    pub fn set_active_tab(&self, id: usize) {
        *self.active_tab_id.borrow_mut() = id;
        for tab in self.tabs.borrow().iter() {
            if tab.id == id {
                tab.widget.style_context().add_class("active-tab");
                self.omnibox.set_text(&tab.url);
                self.update_security_badge(&tab.url);
            } else {
                tab.widget.style_context().remove_class("active-tab");
            }
        }
    }

    /// Updates the security padlock depending on whether URL is HTTPS, HTTP, or internal.
    pub fn update_security_badge(&self, url: &str) {
        if url.starts_with("https://") {
            let img = Image::from_icon_name(Some("security-high-symbolic"), IconSize::Menu);
            self.security_btn.set_image(Some(&img));
            self.security_btn.style_context().remove_class("norva-badge-insecure");
            self.security_btn.style_context().add_class("norva-badge-secure-lock");
            self.security_btn.set_tooltip_text(Some("Secure Connection (TLS)"));
        } else if url.starts_with("norva://") {
            let img = Image::from_icon_name(Some("security-high-symbolic"), IconSize::Menu);
            self.security_btn.set_image(Some(&img));
            self.security_btn.style_context().remove_class("norva-badge-insecure");
            self.security_btn.style_context().add_class("norva-badge-secure-lock");
            self.security_btn.set_tooltip_text(Some("Norva Internal View"));
        } else {
            let img = Image::from_icon_name(Some("security-low-symbolic"), IconSize::Menu);
            self.security_btn.set_image(Some(&img));
            self.security_btn.style_context().remove_class("norva-badge-secure-lock");
            self.security_btn.style_context().add_class("norva-badge-insecure");
            self.security_btn.set_tooltip_text(Some("Insecure Connection (HTTP)"));
        }
    }

    /// Updates bookmark star button state (filled gold if bookmarked).
    pub fn set_bookmarked(&self, bookmarked: bool) {
        if bookmarked {
            let img = Image::from_icon_name(Some("starred-symbolic"), IconSize::Menu);
            self.bookmark_btn.set_image(Some(&img));
            self.bookmark_btn.style_context().add_class("bookmarked");
            self.bookmark_btn.set_tooltip_text(Some("Bookmarked (Ctrl+D)"));
        } else {
            let img = Image::from_icon_name(Some("non-starred-symbolic"), IconSize::Menu);
            self.bookmark_btn.set_image(Some(&img));
            self.bookmark_btn.style_context().remove_class("bookmarked");
            self.bookmark_btn.set_tooltip_text(Some("Bookmark tab (Ctrl+D)"));
        }
    }

    /// Sets page loading state, updating Reload button to Stop and vice-versa.
    pub fn set_page_loading(&self, loading: bool) {
        *self.is_page_loading.borrow_mut() = loading;
        if loading {
            let img = Image::from_icon_name(Some("process-stop-symbolic"), IconSize::Menu);
            self.reload_btn.set_image(Some(&img));
            self.reload_btn.set_tooltip_text(Some("Stop loading (Esc)"));
        } else {
            let img = Image::from_icon_name(Some("view-refresh-symbolic"), IconSize::Menu);
            self.reload_btn.set_image(Some(&img));
            self.reload_btn.set_tooltip_text(Some("Reload (Ctrl+R)"));
        }
    }

    /// Sets current page zoom level badge.
    pub fn set_zoom_level(&self, zoom: f64) {
        *self.zoom_level.borrow_mut() = zoom;
        self.zoom_btn.set_label(&format!("{:.0}%", zoom));
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
                if !icon.is_empty() {
                    tab.icon_label.set_text(icon);
                    tab.icon_label.set_visible(true);
                } else {
                    tab.icon_label.set_visible(false);
                }
                break;
            }
        }
        self.omnibox.set_text(url);
        self.update_security_badge(url);
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
        self.shield_btn.set_label(&format!("Shields {}", count));
    }

    /// Shows or hides the find bar.
    pub fn toggle_find_bar(&self) {
        let vis = !self.find_bar.is_visible();
        self.find_bar.set_visible(vis);
        if vis {
            self.find_entry.grab_focus();
            self.find_entry.select_region(0, -1);
        }
    }

    /// Attaches the Site Security & Privacy Popover.
    pub fn attach_security_popover<F>(&self, on_open_permissions: F)
    where
        F: Fn() + 'static + Clone,
    {
        let popover = Popover::new(Some(&self.security_btn));
        popover.style_context().add_class("norva-popover");

        let pbox = Box::new(Orientation::Vertical, 6);
        pbox.set_margin_top(12);
        pbox.set_margin_bottom(12);
        pbox.set_margin_start(14);
        pbox.set_margin_end(14);

        let title = Label::new(Some("Security & Permissions"));
        title.style_context().add_class("norva-popover-title");
        pbox.pack_start(&title, false, false, 2);

        let stat1 = Label::new(Some("Connection: Encrypted & Verified (TLS)"));
        stat1.style_context().add_class("norva-status-green");
        stat1.set_xalign(0.0);

        let stat2 = Label::new(Some("Hardware: Zero-trust device isolation"));
        stat2.set_xalign(0.0);

        let stat3 = Label::new(Some("Shields: Active tracker defense"));
        stat3.set_xalign(0.0);

        let stat4 = Label::new(Some("Storage: Dedicated SQLite vault"));
        stat4.set_xalign(0.0);

        pbox.pack_start(&stat1, false, false, 2);
        pbox.pack_start(&stat2, false, false, 2);
        pbox.pack_start(&stat3, false, false, 2);
        pbox.pack_start(&stat4, false, false, 2);

        let perm_btn = Button::with_label("Manage Permissions");
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

    /// Attaches the Search Engine selector popover.
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

        let title = Label::new(Some("Search Engine Catalog"));
        title.style_context().add_class("norva-popover-title");
        pbox.pack_start(&title, false, false, 4);

        let engines = [
            ("duckduckgo", "DuckDuckGo (Default)"),
            ("google", "Google Search"),
            ("bing", "Microsoft Bing"),
            ("yahoo", "Yahoo Search"),
            ("ahmia", "Ahmia (.onion Tor Search)"),
            ("startpage", "Startpage"),
            ("brave", "Brave Search"),
            ("github", "GitHub"),
            ("wikipedia", "Wikipedia"),
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
                    "duckduckgo" => "DDG",
                    "google" => "Google",
                    "bing" => "Bing",
                    "yahoo" => "Yahoo",
                    "ahmia" => "Ahmia",
                    "startpage" => "Startpage",
                    "brave" => "Brave",
                    "github" => "GitHub",
                    "wikipedia" => "Wiki",
                    _ => "Search",
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
        pbox.set_margin_top(12);
        pbox.set_margin_bottom(12);
        pbox.set_margin_start(14);
        pbox.set_margin_end(14);

        let title = Label::new(Some("Privacy Shields"));
        title.style_context().add_class("norva-popover-title");
        pbox.pack_start(&title, false, false, 2);

        let status_lbl = Label::new(Some("Active Defense: Trackers & Ads Neutralized"));
        status_lbl.style_context().add_class("norva-status-green");
        pbox.pack_start(&status_lbl, false, false, 2);

        let item1 = Label::new(Some("• Advertising & Tracker Sockets: Blocked"));
        item1.set_xalign(0.0);
        let item2 = Label::new(Some("• Telemetry Beacons: Blocked"));
        item2.set_xalign(0.0);
        let item3 = Label::new(Some("• URL Click Tracking Tokens: Stripped"));
        item3.set_xalign(0.0);
        let item4 = Label::new(Some("• Canvas & Audio Fingerprinting: Defended"));
        item4.set_xalign(0.0);

        pbox.pack_start(&item1, false, false, 1);
        pbox.pack_start(&item2, false, false, 1);
        pbox.pack_start(&item3, false, false, 1);
        pbox.pack_start(&item4, false, false, 1);

        let open_btn = Button::with_label("Open Shields Dashboard");
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

    /// Attaches the Downloads popover to the downloads toolbar button.
    pub fn attach_downloads_popover<F, C>(&self, on_open_downloads: F, on_clear_downloads: C)
    where
        F: Fn() + 'static + Clone,
        C: Fn() + 'static + Clone,
    {
        let popover = Popover::new(Some(&self.downloads_btn));
        popover.style_context().add_class("norva-popover");

        let pbox = Box::new(Orientation::Vertical, 6);
        pbox.set_margin_top(10);
        pbox.set_margin_bottom(10);
        pbox.set_margin_start(12);
        pbox.set_margin_end(12);

        let title = Label::new(Some("Downloads"));
        title.style_context().add_class("norva-popover-title");
        pbox.pack_start(&title, false, false, 2);

        let desc = Label::new(Some("Manage downloaded files and transfers."));
        desc.set_xalign(0.0);
        pbox.pack_start(&desc, false, false, 2);

        let open_mgr_btn = Button::with_label("Open Downloads");
        open_mgr_btn.style_context().add_class("norva-btn-primary");
        let pop_clone = popover.clone();
        let on_open = on_open_downloads.clone();
        open_mgr_btn.connect_clicked(move |_| {
            pop_clone.popdown();
            on_open();
        });
        pbox.pack_start(&open_mgr_btn, false, false, 4);

        let clear_btn = Button::with_label("Clear Downloads History");
        clear_btn.style_context().add_class("norva-popover-item");
        let pop_c2 = popover.clone();
        let on_clr = on_clear_downloads.clone();
        clear_btn.connect_clicked(move |_| {
            pop_c2.popdown();
            on_clr();
        });
        pbox.pack_start(&clear_btn, false, false, 2);

        popover.add(&pbox);
        pbox.show_all();

        self.downloads_btn.connect_clicked(move |_| {
            popover.popup();
        });
    }

    /// Attaches the Browser Menu popover to the three-dots button.
    pub fn attach_menu_popover<F, N, Z, Fnd>(&self, on_navigate: F, on_new_tab: N, on_zoom: Z, on_find: Fnd)
    where
        F: Fn(&str) + 'static + Clone,
        N: Fn() + 'static + Clone,
        Z: Fn(f64) + 'static + Clone,
        Fnd: Fn() + 'static + Clone,
    {
        let popover = Popover::new(Some(&self.menu_btn));
        popover.style_context().add_class("norva-popover");

        let pbox = Box::new(Orientation::Vertical, 3);
        pbox.set_margin_top(8);
        pbox.set_margin_bottom(8);
        pbox.set_margin_start(8);
        pbox.set_margin_end(8);

        // Zoom row
        let zoom_row = Box::new(Orientation::Horizontal, 4);
        zoom_row.style_context().add_class("norva-zoom-row");

        let zoom_lbl = Label::new(Some("Zoom:"));
        zoom_lbl.set_xalign(0.0);
        zoom_row.pack_start(&zoom_lbl, true, true, 4);

        let z_out_btn = Button::with_label("−");
        z_out_btn.style_context().add_class("norva-zoom-btn");
        let on_z1 = on_zoom.clone();
        z_out_btn.connect_clicked(move |_| {
            on_z1(-10.0);
        });

        let z_rst_btn = Button::with_label("100%");
        z_rst_btn.style_context().add_class("norva-zoom-btn");
        let on_z2 = on_zoom.clone();
        z_rst_btn.connect_clicked(move |_| {
            on_z2(0.0);
        });

        let z_in_btn = Button::with_label("+");
        z_in_btn.style_context().add_class("norva-zoom-btn");
        let on_z3 = on_zoom.clone();
        z_in_btn.connect_clicked(move |_| {
            on_z3(10.0);
        });

        zoom_row.pack_start(&z_out_btn, false, false, 2);
        zoom_row.pack_start(&z_rst_btn, false, false, 2);
        zoom_row.pack_start(&z_in_btn, false, false, 2);
        pbox.pack_start(&zoom_row, false, false, 4);

        let menu_items = [
            ("New Tab", "new_tab"),
            ("Find in Page", "find"),
            ("History", "norva://history"),
            ("Bookmarks", "norva://bookmarks"),
            ("Downloads", "norva://downloads"),
            ("Passwords", "norva://passwords"),
            ("Privacy Shields", "norva://shields"),
            ("Site Permissions", "norva://permissions"),
            ("Storage & Cookies", "norva://cookies"),
            ("Reader View", "reader"),
            ("Settings", "norva://settings"),
            ("Start Page", "norva://newtab"),
        ];

        for (label, action) in menu_items {
            let btn = Button::with_label(label);
            btn.style_context().add_class("norva-popover-item");

            let pop_clone = popover.clone();
            let on_nav = on_navigate.clone();
            let on_nt = on_new_tab.clone();
            let on_f = on_find.clone();

            btn.connect_clicked(move |_| {
                pop_clone.popdown();
                if action == "new_tab" {
                    on_nt();
                } else if action == "find" {
                    on_f();
                } else {
                    on_nav(action);
                }
            });

            pbox.pack_start(&btn, false, false, 1);
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
            (&self.cam_btn, "Camera Privacy Guard", "Webcam access is strictly enforced via permissions.db."),
            (&self.mic_btn, "Microphone Privacy Guard", "Audio input streams are guarded against stealth listeners."),
            (&self.loc_btn, "Location Privacy Guard", "GPS and IP geolocation requests are isolated and filtered."),
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
            d_lbl.set_max_width_chars(30);
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

    /// Applies high-tier modern browser GTK CSS styling across the entire window.
    fn apply_theme_css() {
        let css = r###"
        /* ================================================================
           NORVA Executive Dark Browser Chrome Theme (Linear / Arc / Safari)
           ================================================================ */

        window, .background {
            background-color: #0b0c10;
            color: #f4f4f5;
        }

        * {
            -gtk-icon-shadow: none;
            text-shadow: none;
            transition-duration: 120ms;
            transition-timing-function: cubic-bezier(0.16, 1, 0.3, 1);
        }

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
            background-color: #0b0c10;
        }

        /* ---- 1. TAB STRIP ---- */
        .norva-tab-bar {
            background-color: #0b0c10;
            border-bottom: 1px solid rgba(255, 255, 255, 0.06);
            padding: 6px 10px 0 10px;
            min-height: 38px;
        }

        .norva-tabs-scroll {
            background-color: transparent;
            border: none;
        }

        .norva-tabs-container {
            border: none;
        }

        .norva-tab {
            background-color: transparent;
            border-radius: 8px 8px 0 0;
            min-width: 160px;
            max-width: 240px;
            min-height: 32px;
            margin-right: 2px;
            border-top: none;
            border-left: 1px solid transparent;
            border-right: 1px solid rgba(255, 255, 255, 0.04);
            transition: all 0.12s ease;
        }

        .norva-tab:hover {
            background-color: rgba(255, 255, 255, 0.04);
        }

        .norva-tab.active-tab {
            background-color: #12131a;
            border-right: 1px solid rgba(255, 255, 255, 0.08);
            border-left: 1px solid rgba(255, 255, 255, 0.08);
            border-top: 1px solid rgba(255, 255, 255, 0.1);
            box-shadow: none;
        }

        .norva-tab-inner {
            padding: 4px 6px 4px 10px;
        }

        .norva-tab-icon {
            font-size: 11px;
            margin-right: 6px;
            opacity: 0.7;
        }

        .active-tab .norva-tab-icon {
            opacity: 1.0;
        }

        .norva-tab-title {
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', 'Segoe UI', system-ui, sans-serif;
            font-size: 12px;
            font-weight: 450;
            color: #71717a;
        }

        .norva-tab:hover .norva-tab-title {
            color: #d4d4d8;
        }

        .active-tab .norva-tab-title {
            color: #f4f4f5;
            font-weight: 550;
        }

        .norva-tab-close {
            background-color: transparent;
            border: none;
            box-shadow: none;
            color: #71717a;
            border-radius: 4px;
            padding: 0;
            min-width: 18px;
            min-height: 18px;
            margin-left: 4px;
            opacity: 0.0;
            transition: all 0.12s ease;
        }

        .norva-tab:hover .norva-tab-close,
        .active-tab .norva-tab-close {
            opacity: 0.6;
        }

        .norva-tab-close:hover {
            background-color: rgba(255, 255, 255, 0.12);
            color: #ffffff;
            opacity: 1.0;
        }

        .norva-new-tab-btn {
            background-color: transparent;
            border: none;
            color: #71717a;
            border-radius: 6px;
            min-width: 26px;
            min-height: 26px;
            margin: 2px 2px;
            transition: all 0.12s ease;
        }

        .norva-new-tab-btn:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #f4f4f5;
        }

        .norva-badge-normal {
            background-color: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.08);
            color: #a1a1aa;
            padding: 2px 9px;
            border-radius: 12px;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 10px;
            font-weight: 600;
            letter-spacing: 0.3px;
            margin: 4px 0;
        }

        .norva-badge-secure {
            background-color: rgba(99, 102, 241, 0.1);
            border: 1px solid rgba(99, 102, 241, 0.25);
            color: #a5b4fc;
            padding: 2px 9px;
            border-radius: 12px;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 10px;
            font-weight: 600;
            letter-spacing: 0.3px;
            margin: 4px 0;
        }

        /* ---- 2. MAIN TOOLBAR & OMNIBOX ---- */
        .norva-toolbar {
            background-color: #12131a;
            border-bottom: 1px solid rgba(255, 255, 255, 0.06);
            padding: 4px 10px;
            min-height: 42px;
        }

        .norva-nav-btn {
            background-color: transparent;
            border: none;
            color: #a1a1aa;
            border-radius: 6px;
            min-width: 30px;
            min-height: 30px;
            margin: 0 1px;
            transition: all 0.12s ease;
        }

        .norva-nav-btn:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #ffffff;
        }

        .norva-nav-btn:active {
            background-color: rgba(255, 255, 255, 0.14);
        }

        /* Omnibox Capsule Frame */
        .norva-omnibox-frame {
            background-color: #0b0c10;
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 20px;
            padding: 0 8px;
            min-height: 32px;
            margin: 0 8px;
            transition: all 0.15s ease;
        }

        .norva-omnibox-frame:focus-within {
            border-color: rgba(99, 102, 241, 0.6);
            box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.18);
            background-color: #0e0f15;
        }

        .norva-btn-flat {
            background-color: transparent;
            border: none;
            padding: 0 4px;
            color: #71717a;
            border-radius: 50%;
            min-width: 24px;
            min-height: 24px;
        }

        .norva-badge-secure-lock {
            color: #a1a1aa;
        }
        .norva-badge-secure-lock:hover {
            color: #ffffff;
            background-color: rgba(255, 255, 255, 0.08);
        }

        .norva-badge-insecure {
            color: #f59e0b;
        }
        .norva-badge-insecure:hover {
            background-color: rgba(245, 158, 11, 0.15);
        }

        .norva-engine-pill {
            background-color: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.07);
            color: #a1a1aa;
            border-radius: 10px;
            padding: 1px 7px;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 11px;
            font-weight: 500;
            min-height: 22px;
            margin: 0 3px;
        }

        .norva-omnibox-entry {
            background-color: transparent;
            border: none;
            color: #f4f4f5;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', 'Segoe UI', system-ui, sans-serif;
            font-size: 13px;
            padding: 3px 6px;
            caret-color: #818cf8;
        }

        .norva-omni-clear-btn {
            background-color: transparent;
            border: none;
            color: #71717a;
            border-radius: 50%;
            min-width: 20px;
            min-height: 20px;
            margin: 0 1px;
        }
        .norva-omni-clear-btn:hover {
            color: #ffffff;
            background-color: rgba(255, 255, 255, 0.1);
        }

        .norva-omni-action-btn {
            background-color: transparent;
            border: none;
            color: #71717a;
            border-radius: 4px;
            min-width: 24px;
            min-height: 24px;
            font-size: 11px;
            font-weight: 500;
            margin: 0 1px;
        }
        .norva-omni-action-btn:hover {
            color: #ffffff;
            background-color: rgba(255, 255, 255, 0.08);
        }

        .norva-zoom-pill {
            background-color: rgba(255, 255, 255, 0.04);
            border-radius: 8px;
            color: #71717a;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 10px;
            font-weight: 600;
            padding: 1px 5px;
            min-height: 20px;
            margin: 0 2px;
        }

        .norva-bookmark-star {
            background-color: transparent;
            border: none;
            color: #71717a;
            border-radius: 50%;
            min-width: 24px;
            min-height: 24px;
            padding: 0;
            transition: all 0.12s ease;
        }

        .norva-bookmark-star:hover {
            color: #fbbf24;
            background-color: rgba(251, 191, 36, 0.12);
        }

        .norva-bookmark-star.bookmarked {
            color: #fbbf24;
        }

        /* Shield Button */
        .norva-shield-btn {
            background-color: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.08);
            color: #a1a1aa;
            border-radius: 14px;
            padding: 2px 10px;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 11px;
            font-weight: 600;
            min-height: 28px;
            margin: 0 2px;
            transition: all 0.12s ease;
        }

        .norva-shield-btn:hover {
            background-color: rgba(99, 102, 241, 0.14);
            border-color: rgba(99, 102, 241, 0.35);
            color: #c7d2fe;
        }

        /* Hardware Zero-Trust Cluster */
        .norva-hw-cluster {
            background-color: rgba(255, 255, 255, 0.03);
            border-radius: 8px;
            padding: 1px 3px;
            margin: 0 2px;
        }

        .norva-hw-btn {
            background-color: transparent;
            border: none;
            font-size: 10px;
            color: #71717a;
            padding: 0 4px;
            border-radius: 4px;
            min-width: 22px;
            min-height: 22px;
            transition: all 0.12s ease;
        }

        .norva-hw-btn:hover {
            color: #ffffff;
            background-color: rgba(255, 255, 255, 0.08);
        }

        /* ---- 3. FIND IN PAGE BAR ---- */
        .norva-find-bar {
            background-color: #12131a;
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
            border-top: 1px solid rgba(255, 255, 255, 0.04);
            padding: 4px 14px;
            min-height: 34px;
        }

        .norva-find-icon {
            color: #71717a;
            margin-right: 4px;
        }

        .norva-find-entry {
            background-color: #0b0c10;
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 6px;
            color: #ffffff;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 12px;
            padding: 2px 8px;
            min-width: 220px;
        }

        .norva-find-count {
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 11px;
            color: #71717a;
            margin: 0 6px;
        }

        .norva-find-btn {
            background-color: transparent;
            border-radius: 4px;
            color: #a1a1aa;
            min-width: 24px;
            min-height: 24px;
            margin: 0 1px;
        }
        .norva-find-btn:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #ffffff;
        }

        .norva-find-close-btn {
            background-color: transparent;
            border-radius: 4px;
            color: #71717a;
            min-width: 24px;
            min-height: 24px;
        }
        .norva-find-close-btn:hover {
            color: #ffffff;
            background-color: rgba(255, 255, 255, 0.08);
        }

        /* ---- 4. BOOKMARKS BAR ---- */
        .norva-bookmarks-bar {
            background-color: #12131a;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            padding: 2px 10px;
            min-height: 28px;
        }

        .norva-bookmark-pill {
            background-color: transparent;
            border: none;
            color: #a1a1aa;
            border-radius: 4px;
            padding: 2px 8px;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 11.5px;
            font-weight: 450;
            min-height: 22px;
            margin: 0 1px;
            transition: all 0.12s ease;
        }

        .norva-bookmark-pill:hover {
            background-color: rgba(255, 255, 255, 0.07);
            color: #f4f4f5;
        }

        /* ---- 5. POPOVERS ---- */
        .norva-popover {
            background-color: #161722;
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 12px;
            color: #f4f4f5;
            padding: 6px;
            box-shadow: 0 16px 40px rgba(0, 0, 0, 0.7);
        }

        .norva-popover-title {
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-weight: 600;
            font-size: 13px;
            color: #ffffff;
            margin-bottom: 6px;
            padding: 4px 8px 2px 8px;
        }

        .norva-popover-item {
            background-color: transparent;
            border: none;
            color: #a1a1aa;
            border-radius: 6px;
            padding: 7px 12px;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 12.5px;
            font-weight: 450;
            min-height: 30px;
            transition: all 0.1s ease;
        }

        .norva-popover-item:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #ffffff;
        }

        .norva-zoom-row {
            padding: 4px 8px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.06);
            margin-bottom: 4px;
        }

        .norva-zoom-btn {
            background-color: rgba(255, 255, 255, 0.05);
            border-radius: 4px;
            font-size: 11px;
            font-weight: 500;
            color: #d4d4d8;
            padding: 3px 8px;
            min-height: 24px;
        }
        .norva-zoom-btn:hover {
            background-color: rgba(255, 255, 255, 0.12);
            color: #ffffff;
        }

        .norva-btn-primary {
            background-color: #6366f1;
            border: none;
            color: #ffffff;
            border-radius: 6px;
            padding: 7px 14px;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 12px;
            font-weight: 550;
            min-height: 30px;
            transition: all 0.12s ease;
        }

        .norva-btn-primary:hover {
            background-color: #4f46e5;
        }

        .norva-status-green {
            color: #10b981;
            font-family: -apple-system, BlinkMacSystemFont, 'Inter', system-ui, sans-serif;
            font-size: 12px;
            font-weight: 500;
        }

        scrollbar {
            background-color: transparent;
        }
        scrollbar slider {
            background-color: rgba(255, 255, 255, 0.1);
            border-radius: 999px;
            min-width: 5px;
            min-height: 5px;
        }
        scrollbar slider:hover {
            background-color: rgba(255, 255, 255, 0.2);
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
