// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/ui/assets.rs
// Description: Modern, Dark-Mode Glassmorphic UI Pages for internal norva:// views.
// ============================================================================

use crate::db::history_db::HistoryEntry;
use crate::db::permissions_db::{PermissionAuditLog, SitePermission};
use crate::db::settings_db::SearchEngineConfig;
use crate::db::shields_db::{BlockedEvent, ShieldStats};

/// Shared CSS styling providing an ultra-sleek, futuristic, glassmorphic dark theme.
/// Shared CSS styling providing an authentic, modern Firefox Proton dark theme.
pub fn get_common_css() -> &'static str {
    r###"
    :root {
        --bg-primary: #16171b;
        --bg-card: #212228;
        --bg-card-hover: #292a32;
        --bg-subtle: #30313c;
        --border-subtle: rgba(255, 255, 255, 0.06);
        --border-hover: rgba(255, 255, 255, 0.14);
        --border-focus: #5884ff;
        --accent-blue: #5884ff;
        --accent-purple: #9059ff;
        --accent-green: #50c878;
        --accent-red: #ff5252;
        --text-primary: #ededf2;
        --text-secondary: #9d9da8;
        --text-muted: #6e6e7a;
    }
    * {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }
    body {
        font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
        background-color: var(--bg-primary);
        color: var(--text-primary);
        min-height: 100vh;
        overflow-x: hidden;
        padding-bottom: 40px;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    /* Scrollbar styling */
    ::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    ::-webkit-scrollbar-track {
        background: transparent;
    }
    ::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 999px;
    }
    ::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.2);
    }

    /* Internal Views Navigation Bar */
    .internal-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 14px 28px;
        background-color: #212228;
        border-bottom: 1px solid var(--border-subtle);
        position: sticky;
        top: 0;
        z-index: 100;
        flex-wrap: wrap;
        gap: 12px;
        backdrop-filter: blur(12px);
    }
    .internal-header-left {
        display: flex;
        align-items: center;
        gap: 14px;
    }
    .back-link {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        background-color: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-subtle);
        color: var(--text-secondary);
        padding: 7px 14px;
        border-radius: 8px;
        text-decoration: none;
        font-size: 13px;
        font-weight: 500;
        transition: all 0.15s ease;
    }
    .back-link:hover {
        background-color: rgba(255, 255, 255, 0.1);
        color: #ffffff;
    }
    .internal-title {
        font-size: 17px;
        font-weight: 700;
        color: var(--text-primary);
        letter-spacing: -0.3px;
    }
    .internal-nav-chips {
        display: flex;
        align-items: center;
        gap: 6px;
        flex-wrap: wrap;
    }
    .chip {
        color: var(--text-secondary);
        text-decoration: none;
        padding: 6px 14px;
        border-radius: 8px;
        font-size: 13px;
        font-weight: 500;
        transition: all 0.15s ease;
        background: transparent;
    }
    .chip:hover {
        background-color: rgba(255, 255, 255, 0.06);
        color: #ffffff;
    }
    .chip.active {
        background-color: rgba(88, 132, 255, 0.12);
        color: #ffffff;
        font-weight: 600;
    }
    .container {
        max-width: 1120px;
        margin: 0 auto;
        padding: 28px 24px;
    }
    .btn {
        background-color: rgba(255, 255, 255, 0.06);
        border: 1px solid var(--border-subtle);
        color: var(--text-primary);
        border-radius: 8px;
        padding: 8px 16px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        display: inline-flex;
        align-items: center;
        gap: 6px;
    }
    .btn:hover {
        background-color: rgba(255, 255, 255, 0.1);
        border-color: rgba(255, 255, 255, 0.12);
    }
    .btn-primary {
        background-color: var(--accent-blue);
        border: none;
        color: #ffffff;
    }
    .btn-primary:hover {
        background-color: #4a72e8;
    }
    .btn-danger {
        background-color: rgba(255, 82, 82, 0.1);
        border: 1px solid rgba(255, 82, 82, 0.25);
        color: #ff8585;
    }
    .btn-danger:hover {
        background-color: rgba(255, 82, 82, 0.2);
    }
    .card {
        background-color: var(--bg-card);
        border: 1px solid var(--border-subtle);
        border-radius: 14px;
        padding: 22px;
        margin-bottom: 20px;
        transition: border-color 0.15s ease;
    }
    .card:hover {
        border-color: rgba(255, 255, 255, 0.1);
    }
    .mode-badge {
        padding: 4px 14px;
        border-radius: 20px;
        font-size: 11px;
        font-weight: 600;
        letter-spacing: 0.5px;
        display: inline-block;
    }
    .mode-badge.secure {
        background: rgba(255, 82, 82, 0.1);
        color: #ff7070;
        border: 1px solid rgba(255, 82, 82, 0.22);
    }
    .mode-badge.normal {
        background: rgba(80, 200, 120, 0.08);
        color: #50c878;
        border: 1px solid rgba(80, 200, 120, 0.2);
    }
    "###
}

/// Renders the primary `norva://newtab` start page directly matching modern Firefox (Proton Dark).
pub fn render_newtab_page(
    stats: &ShieldStats,
    engines: &[SearchEngineConfig],
    default_engine: &SearchEngineConfig,
    secure_mode: bool,
    cache_mb: f64,
) -> String {
    let mut engine_items_html = String::new();
    for e in engines {
        let is_sel = if e.id == default_engine.id { "active" } else { "" };
        let icon = match e.id.as_str() {
            "duckduckgo" => "🦆",
            "google" => "🔍",
            "bing" => "⚡",
            "yahoo" => "🟣",
            "ahmia" => "🧅",
            "startpage" => "🛡️",
            "brave" => "🦁",
            _ => "🌐",
        };
        engine_items_html.push_str(&format!(
            r###"<div class="engine-dropdown-item {}" data-id="{}" data-name="{}" data-bang="{}" data-icon="{}" onclick="selectSearchEngine('{}', '{}', '{}', '{}')">
                <span class="engine-item-icon">{}</span>
                <span class="engine-item-name">{}</span>
                <span class="engine-item-bang">{}</span>
            </div>"###,
            is_sel, e.id, e.name, e.bang, icon, e.id, e.name, e.bang, icon, icon, e.name, e.bang
        ));
    }

    let default_icon = match default_engine.id.as_str() {
        "duckduckgo" => "🦆",
        "google" => "🔍",
        "bing" => "⚡",
        "yahoo" => "🟣",
        "ahmia" => "🧅",
        _ => "🌐",
    };

    let mode_pill = if secure_mode {
        r###"<span class="mode-badge secure">🔒 Private Ghost Session</span>"###
    } else {
        r###"<span class="mode-badge normal">🛡️ Hardened Session</span>"###
    };

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>New Tab</title>
    <style>
        {{COMMON_CSS}}

        /* Import Inter font */
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap');

        /* Modern Browser Canvas */
        .nt-canvas {
            max-width: 1260px;
            margin: 0 auto;
            padding: 30px 36px 80px;
            position: relative;
        }

        /* Top Brand Header */
        .nt-topbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 28px;
        }
        .nt-brand {
            display: flex;
            align-items: center;
            gap: 12px;
            text-decoration: none;
            color: inherit;
        }
        .nt-brand-text {
            font-size: 26px;
            font-weight: 800;
            letter-spacing: -0.8px;
            color: #e8e8ed;
        }
        .nt-topbar-actions {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        .customize-top-btn {
            background-color: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.08);
            color: #a0a0aa;
            padding: 7px 16px;
            border-radius: 20px;
            font-size: 13px;
            font-weight: 500;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
            transition: all 0.15s ease;
        }
        .customize-top-btn:hover {
            background-color: rgba(255, 255, 255, 0.1);
            color: #ffffff;
        }

        /* Centered Sleek Search Bar (Firefox & Chrome style, NO clunky select tag) */
        .nt-search-container {
            max-width: 720px;
            margin: 0 auto 36px;
        }
        .nt-search-box {
            display: flex;
            align-items: center;
            background-color: #24232c;
            border: 1.5px solid rgba(255, 255, 255, 0.08);
            border-radius: 28px;
            height: 52px;
            padding: 0 18px;
            transition: all 0.2s ease;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
            position: relative;
        }
        .nt-search-box:focus-within {
            border-color: rgba(88, 132, 255, 0.5);
            box-shadow: 0 0 0 2px rgba(88, 132, 255, 0.15), 0 6px 24px rgba(0, 0, 0, 0.4);
            background-color: #2a2935;
        }
        .engine-picker-btn {
            display: flex;
            align-items: center;
            gap: 6px;
            background: rgba(255, 255, 255, 0.06);
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 18px;
            padding: 4px 10px;
            color: #fbfbfe;
            cursor: pointer;
            font-size: 13px;
            font-weight: 500;
            margin-right: 12px;
            transition: background 0.15s ease;
            user-select: none;
        }
        .engine-picker-btn:hover {
            background: rgba(255, 255, 255, 0.12);
        }
        .engine-dropdown-menu {
            display: none;
            position: absolute;
            top: 58px;
            left: 12px;
            width: 260px;
            background-color: #28272f;
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 14px;
            box-shadow: 0 12px 40px rgba(0, 0, 0, 0.6);
            z-index: 200;
            overflow: hidden;
            padding: 6px;
        }
        .engine-dropdown-menu.show {
            display: block;
        }
        .engine-dropdown-item {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 9px 14px;
            border-radius: 10px;
            color: #b0b0ba;
            font-size: 13px;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .engine-dropdown-item:hover, .engine-dropdown-item.active {
            background-color: rgba(88, 132, 255, 0.12);
            color: #ffffff;
        }
        .engine-item-icon {
            font-size: 16px;
        }
        .engine-item-name {
            flex: 1;
            font-weight: 500;
        }
        .engine-item-bang {
            font-size: 11px;
            color: #6e6e7a;
            background: rgba(0, 0, 0, 0.25);
            padding: 2px 6px;
            border-radius: 4px;
        }
        .nt-search-input {
            flex: 1;
            background: transparent;
            border: none;
            color: #e8e8ed;
            font-size: 15px;
            outline: none;
        }
        .nt-search-input::placeholder {
            color: #8f8f9d;
        }
        .nt-search-actions {
            display: flex;
            align-items: center;
            gap: 4px;
        }
        .nt-clear-btn {
            background: transparent;
            border: none;
            color: #8f8f9d;
            cursor: pointer;
            padding: 4px;
            font-size: 14px;
            display: none;
        }
        .nt-clear-btn:hover {
            color: #fbfbfe;
        }
        .nt-search-btn {
            background: transparent;
            border: none;
            color: #cfcfd4;
            cursor: pointer;
            padding: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 50%;
            transition: background-color 0.15s ease, color 0.15s ease;
        }
        .nt-search-btn:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #ffffff;
        }

        /* Search Quick Bang Chips (Chrome & Firefox style) */
        .search-chips-row {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            margin-top: 10px;
            flex-wrap: wrap;
        }
        .search-chip {
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.06);
            border-radius: 16px;
            padding: 4px 12px;
            font-size: 11px;
            color: #8e8e98;
            cursor: pointer;
            transition: all 0.15s ease;
            text-decoration: none;
        }
        .search-chip:hover {
            background: rgba(255, 255, 255, 0.08);
            color: #ffffff;
            border-color: rgba(255, 255, 255, 0.2);
        }

        /* Top Sites / Shortcuts Row (Firefox 48x48 rounded squircle tiles) */
        .shortcuts-section {
            margin-bottom: 44px;
        }
        .shortcuts-row {
            display: flex;
            align-items: flex-start;
            justify-content: center;
            gap: 22px;
            flex-wrap: wrap;
        }
        .shortcut-item {
            display: flex;
            flex-direction: column;
            align-items: center;
            text-decoration: none;
            color: inherit;
            width: 78px;
            cursor: pointer;
            position: relative;
        }
        .shortcut-tile {
            width: 52px;
            height: 52px;
            border-radius: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 22px;
            margin-bottom: 8px;
            transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1), box-shadow 0.2s ease;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
            position: relative;
        }
        .shortcut-item:hover .shortcut-tile {
            transform: translateY(-3px) scale(1.04);
            box-shadow: 0 8px 20px rgba(0, 0, 0, 0.45);
        }
        .shortcut-menu-trigger {
            display: none;
            position: absolute;
            top: -6px;
            right: 8px;
            width: 20px;
            height: 20px;
            border-radius: 50%;
            background-color: #38374a;
            color: #ffffff;
            font-size: 10px;
            align-items: center;
            justify-content: center;
            box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
            cursor: pointer;
        }
        .shortcut-item:hover .shortcut-menu-trigger {
            display: flex;
        }
        .shortcut-label {
            font-size: 12px;
            font-weight: 500;
            color: #d1d1d6;
            text-align: center;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: 78px;
            line-height: 1.2;
        }
        .shortcut-sub {
            font-size: 10px;
            color: #6e6e7a;
            text-align: center;
            margin-top: 2px;
        }

        /* Widgets Section (Accordion Header + 4 Cards Grid) */
        .section-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 16px;
            position: relative;
        }
        .section-title {
            display: flex;
            align-items: center;
            letter-spacing: -0.3px;
            gap: 8px;
            font-size: 16px;
            font-weight: 700;
            color: #fbfbfe;
            cursor: pointer;
            user-select: none;
        }
        .section-menu-btn {
            color: #6e6e7a;
            cursor: pointer;
            font-size: 16px;
            background: none;
            border: none;
            padding: 4px 8px;
            border-radius: 4px;
            transition: background 0.15s ease;
        }
        .section-menu-btn:hover {
            background-color: #2b2a33;
            color: #fbfbfe;
        }
        .section-popover {
            display: none;
            position: absolute;
            right: 0;
            top: 32px;
            width: 220px;
            background-color: #28272f;
            border: 1px solid rgba(255, 255, 255, 0.15);
            border-radius: 12px;
            padding: 6px;
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
            z-index: 150;
        }
        .section-popover.show {
            display: block;
        }
        .popover-item {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 9px 14px;
            border-radius: 8px;
            font-size: 13px;
            color: #cfcfd4;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .popover-item:hover {
            background-color: rgba(88, 132, 255, 0.12);
            color: #ffffff;
        }

        .widgets-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 16px;
            margin-bottom: 44px;
        }
        @media (max-width: 1024px) {
            .widgets-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        @media (max-width: 640px) {
            .widgets-grid {
                grid-template-columns: 1fr;
            }
        }

        .widget-card {
            background-color: #24232c;
            border-radius: 14px;
            padding: 18px;
            min-height: 120px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            border: 1px solid rgba(255, 255, 255, 0.05);
            position: relative;
            transition: border-color 0.15s ease;
        }
        .widget-card:hover {
            border-color: rgba(255, 255, 255, 0.1);
        }

        /* 1. World Clock Card */
        .clock-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            height: 100%;
            text-align: center;
        }
        .clock-col {
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            padding: 0 4px;
            border-right: 1px solid rgba(255, 255, 255, 0.08);
            cursor: pointer;
        }
        .clock-col:last-child {
            border-right: none;
        }
        .clock-city {
            font-size: 11.5px;
            font-weight: 700;
            color: #fbfbfe;
        }
        .clock-tz {
            font-size: 9px;
            color: #6e6e7a;
            margin-top: 1px;
        }
        .clock-time {
            font-size: 12px;
            font-weight: 700;
            color: #fbfbfe;
            margin-top: 12px;
        }

        /* 2. Weather Card */
        .weather-top {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 8px;
        }
        .weather-loc {
            font-size: 12px;
            font-weight: 600;
            color: #fbfbfe;
            cursor: pointer;
        }
        .weather-main {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        .weather-temp {
            font-size: 18px;
            font-weight: 700;
            color: #fbfbfe;
        }
        .weather-cond {
            font-size: 11px;
            color: #8f8f9d;
        }
        .weather-forecast {
            display: flex;
            gap: 12px;
            font-size: 10px;
            color: #cfcfd4;
            border-top: 1px solid rgba(255, 255, 255, 0.06);
            padding-top: 6px;
            margin-top: 8px;
        }

        /* 3. Focus / Pomodoro Timer Card */
        .timer-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
        }
        .timer-play-btn {
            width: 38px;
            height: 38px;
            border-radius: 50%;
            background-color: #38374a;
            border: none;
            color: #e8e8ed;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 14px;
            transition: background-color 0.15s ease;
        }
        .timer-play-btn:hover {
            background-color: #4a4960;
        }
        .timer-digits {
            font-size: 18px;
            font-weight: 700;
            color: #e8e8ed;
            letter-spacing: 0.5px;
        }
        .timer-ctrl-btn {
            background: none;
            border: none;
            color: #6e6e7a;
            font-size: 16px;
            cursor: pointer;
            padding: 0 4px;
        }
        .timer-ctrl-btn:hover {
            color: #e8e8ed;
        }
        .timer-pills {
            display: flex;
            gap: 6px;
            margin-top: 8px;
        }
        .timer-pill {
            background: transparent;
            border: 1px solid rgba(255, 255, 255, 0.15);
            color: #a0a0aa;
            padding: 3px 12px;
            border-radius: 12px;
            font-size: 11px;
            cursor: pointer;
            font-weight: 500;
        }
        .timer-pill.active {
            background-color: rgba(88, 132, 255, 0.15);
            border-color: rgba(88, 132, 255, 0.25);
            color: #ffffff;
            font-weight: 600;
        }

        /* 4. Checklist Card */
        .checklist-title {
            font-size: 12.5px;
            font-weight: 700;
            color: #a0a0aa;
            margin-bottom: 6px;
        }
        .checklist-add-btn {
            background-color: #9059ff;
            border: none;
            color: #ffffff;
            padding: 5px 12px;
            border-radius: 14px;
            font-size: 12px;
            font-weight: 600;
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            gap: 4px;
            transition: background-color 0.15s ease;
        }
        .checklist-add-btn:hover {
            background-color: #804cee;
        }
        .checklist-icon-decor {
            position: absolute;
            right: 14px;
            bottom: 12px;
            opacity: 0.9;
        }
        .checklist-items-box {
            margin-top: 6px;
            max-height: 52px;
            overflow-y: auto;
            font-size: 11px;
            color: #cfcfd4;
        }
        .checklist-item-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 2px 0;
        }

        /* Thought-provoking stories (News Section with REAL Photography & Controls) */
        .stories-section {
            margin-bottom: 40px;
        }
        .stories-category-chips {
            display: flex;
            gap: 6px;
            overflow-x: auto;
            padding-bottom: 8px;
            margin-bottom: 14px;
        }
        .category-chip {
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.06);
            color: #8e8e98;
            padding: 5px 14px;
            border-radius: 18px;
            font-size: 12px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.15s ease;
            white-space: nowrap;
        }
        .category-chip:hover, .category-chip.active {
            background: rgba(88, 132, 255, 0.12);
            color: #ffffff;
            border-color: rgba(255, 255, 255, 0.2);
        }

        .stories-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 16px;
        }
        @media (max-width: 1024px) {
            .stories-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        @media (max-width: 640px) {
            .stories-grid {
                grid-template-columns: 1fr;
            }
        }
        .story-card {
            background-color: #24232c;
            border-radius: 14px;
            overflow: hidden;
            text-decoration: none;
            color: inherit;
            display: flex;
            flex-direction: column;
            transition: transform 0.15s ease, background-color 0.15s ease;
            border: 1px solid rgba(255, 255, 255, 0.05);
            cursor: pointer;
        }
        .story-card:hover {
            transform: translateY(-4px);
            background-color: #2a2935;
            box-shadow: 0 12px 32px rgba(0, 0, 0, 0.4);
            border-color: rgba(255, 255, 255, 0.1);
        }
        .story-thumb-wrap {
            width: 100%;
            height: 160px;
            position: relative;
            background-color: #17161d;
            overflow: hidden;
        }
        .story-thumb {
            width: 100%;
            height: 100%;
            object-fit: cover;
            display: block;
            transition: transform 0.3s ease;
        }
        .story-card:hover .story-thumb {
            transform: scale(1.03);
        }
        .story-badge-corner {
            position: absolute;
            top: 8px;
            left: 8px;
            background: rgba(0, 0, 0, 0.75);
            backdrop-filter: blur(4px);
            color: #ffffff;
            font-size: 10px;
            font-weight: 700;
            padding: 2px 6px;
            border-radius: 4px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .story-content {
            padding: 14px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            flex: 1;
        }
        .story-title {
            font-size: 13.5px;
            font-weight: 600;
            color: #fbfbfe;
            line-height: 1.4;
            margin-bottom: 12px;
            display: -webkit-box;
            -webkit-line-clamp: 3;
            -webkit-box-orient: vertical;
            overflow: hidden;
        }
        .story-footer {
            margin-top: auto;
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-size: 11px;
            color: #8f8f9d;
        }
        .story-source {
            display: flex;
            align-items: center;
            gap: 6px;
            font-weight: 500;
        }

        /* Subtle Privacy & Internal Vaults Bar */
        .vault-pills-bar {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 10px;
            flex-wrap: wrap;
            padding: 16px 0;
            border-top: 1px solid rgba(255, 255, 255, 0.06);
            margin-top: 16px;
        }
        .vault-pill-link {
            background-color: rgba(255, 255, 255, 0.04);
            color: #8e8e98;
            text-decoration: none;
            padding: 6px 14px;
            border-radius: 18px;
            font-size: 12px;
            font-weight: 500;
            transition: all 0.15s ease;
        }
        .vault-pill-link:hover {
            background-color: rgba(255, 255, 255, 0.08);
            color: #e0e0e6;
        }

        /* Bottom Right Customise Button (Matching Chrome Image 2 & Firefox Image 1) */
        .customize-floating-pill {
            position: fixed;
            right: 24px;
            bottom: 24px;
            background-color: #28272f;
            border: 1px solid rgba(255, 255, 255, 0.1);
            color: #d1d1d6;
            padding: 9px 18px;
            border-radius: 22px;
            font-size: 13px;
            font-weight: 500;
            display: flex;
            align-items: center;
            gap: 8px;
            cursor: pointer;
            box-shadow: 0 6px 24px rgba(0, 0, 0, 0.5);
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
            z-index: 99;
        }
        .customize-floating-pill:hover {
            background-color: #32313c;
            transform: translateY(-3px);
            box-shadow: 0 10px 32px rgba(0, 0, 0, 0.6);
        }

        /* Comprehensive Homepage Customization Drawer */
        .customizer-drawer-overlay {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100vh;
            background: rgba(0, 0, 0, 0.5);
            z-index: 300;
        }
        .customizer-drawer {
            position: fixed;
            top: 0;
            right: -360px;
            width: 360px;
            height: 100vh;
            background-color: #1e1d27;
            border-left: 1px solid rgba(255, 255, 255, 0.08);
            box-shadow: -12px 0 48px rgba(0, 0, 0, 0.65);
            z-index: 301;
            transition: right 0.25s cubic-bezier(0.16, 1, 0.3, 1);
            display: flex;
            flex-direction: column;
        }
        .customizer-drawer.open {
            right: 0;
        }
        .drawer-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 18px 20px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
        }
        .drawer-title {
            font-size: 16px;
            font-weight: 700;
            color: #fbfbfe;
        }
        .drawer-close-btn {
            background: none;
            border: none;
            color: #8f8f9d;
            font-size: 20px;
            cursor: pointer;
            padding: 4px;
        }
        .drawer-close-btn:hover {
            color: #fbfbfe;
        }
        .drawer-body {
            padding: 20px;
            overflow-y: auto;
            flex: 1;
        }
        .drawer-section {
            margin-bottom: 24px;
        }
        .drawer-section-title {
            font-size: 12px;
            font-weight: 700;
            color: #8f8f9d;
            text-transform: uppercase;
            letter-spacing: 0.8px;
            margin-bottom: 12px;
        }
        .theme-swatches-grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 10px;
            margin-bottom: 16px;
        }
        .theme-swatch {
            border-radius: 8px;
            height: 48px;
            border: 2px solid transparent;
            cursor: pointer;
            position: relative;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 10px;
            font-weight: 600;
            color: #ffffff;
            transition: transform 0.15s ease, border-color 0.15s ease;
        }
        .theme-swatch:hover, .theme-swatch.active {
            border-color: #0060df;
            transform: scale(1.04);
        }
        .drawer-toggle-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 10px 0;
            border-bottom: 1px solid rgba(255, 255, 255, 0.06);
            font-size: 13.5px;
            color: #fbfbfe;
        }
        .toggle-switch {
            position: relative;
            display: inline-block;
            width: 36px;
            height: 20px;
        }
        .toggle-switch input {
            opacity: 0;
            width: 0;
            height: 0;
        }
        .slider {
            position: absolute;
            cursor: pointer;
            border-radius: 20px;
            top: 0; left: 0; right: 0; bottom: 0;
            background-color: #38374a;
            transition: .2s;
            border-radius: 20px;
        }
        .slider:before {
            position: absolute;
            content: "";
            height: 14px;
            width: 14px;
            left: 3px;
            bottom: 3px;
            background-color: white;
            transition: .2s;
            border-radius: 50%;
        }
        input:checked + .slider {
            background-color: #5884ff;
        }
        input:checked + .slider:before {
            transform: translateX(16px);
        }

        /* Modal Dialog for Adding Shortcuts */
        .modal-overlay {
            display: none;
            position: fixed;
            top: 0; left: 0; width: 100vw; height: 100vh;
            background: rgba(0, 0, 0, 0.6);
            z-index: 400;
            align-items: center;
            justify-content: center;
        }
        .modal-overlay.show {
            display: flex;
        }
        .modal-box {
            background-color: #28272f;
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 16px;
            width: 400px;
            padding: 28px;
            box-shadow: 0 16px 48px rgba(0, 0, 0, 0.65);
        }
        .modal-title {
            font-size: 16px;
            font-weight: 700;
            color: #fbfbfe;
            margin-bottom: 16px;
        }
        .modal-input {
            width: 100%;
            background-color: #1e1d27;
            border: 1.5px solid rgba(255, 255, 255, 0.08);
            border-radius: 10px;
            padding: 11px 14px;
            color: #e8e8ed;
            font-size: 14px;
            outline: none;
            margin-bottom: 12px;
            box-sizing: border-box;
        }
        .modal-input:focus {
            border-color: rgba(88, 132, 255, 0.5);
            box-shadow: 0 0 0 2px rgba(88, 132, 255, 0.12);
        }
        .modal-actions {
            display: flex;
            justify-content: flex-end;
            gap: 10px;
            margin-top: 16px;
        }
    </style>
</head>
<body>
    <div class="nt-canvas" id="canvas-root">
        <!-- Topbar (Brand + Header Actions) -->
        <div class="nt-topbar">
            <a href="norva://newtab" class="nt-brand">
                <svg width="40" height="40" viewBox="0 0 100 100" fill="none">
                    <defs>
                        <radialGradient id="globeGrad" cx="45%" cy="45%" r="60%">
                            <stop offset="0%" stop-color="#4a3b89"/>
                            <stop offset="100%" stop-color="#1c1b22"/>
                        </radialGradient>
                        <linearGradient id="flameGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                            <stop offset="0%" stop-color="#ff7139"/>
                            <stop offset="45%" stop-color="#ff3b30"/>
                            <stop offset="100%" stop-color="#9059ff"/>
                        </linearGradient>
                    </defs>
                    <circle cx="50" cy="50" r="38" fill="url(#globeGrad)"/>
                    <path d="M50 14 C66 24 84 40 84 62 C84 78 72 88 56 88 C42 88 32 78 32 68 C32 54 44 46 48 34 C34 40 22 54 22 70 C22 84 34 92 50 92 C72 92 88 76 88 56 C88 36 72 20 50 14 Z" fill="url(#flameGrad)"/>
                </svg>
                <span class="nt-brand-text">norva</span>
            </a>
            <div class="nt-topbar-actions">
                {{MODE_PILL}}
                <button class="customize-top-btn" onclick="openCustomizer()">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="3"/>
                        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
                    </svg>
                    <span>Customise</span>
                </button>
            </div>
        </div>

        <!-- Centered Modern Search Bar -->
        <div class="nt-search-container">
            <div class="nt-search-box">
                <!-- Clean Search Engine Picker Trigger (Replaces ugly <select>) -->
                <div class="engine-picker-btn" id="engine-picker-trigger" onclick="toggleEngineDropdown()">
                    <span id="active-engine-icon">{{DEFAULT_ICON}}</span>
                    <span id="active-engine-label">{{DEFAULT_NAME}}</span>
                    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                        <polyline points="6 9 12 15 18 9"/>
                    </svg>
                </div>

                <!-- Floating Engine Dropdown -->
                <div class="engine-dropdown-menu" id="engine-dropdown">
                    {{ENGINE_ITEMS}}
                </div>

                <input type="text" class="nt-search-input" id="search-input" 
                    placeholder="Search with {{DEFAULT_NAME}} or enter address" autocomplete="off" autofocus />
                
                <div class="nt-search-actions">
                    <button class="nt-clear-btn" id="search-clear" onclick="clearSearchInput()">✕</button>
                    <button class="nt-search-btn" id="search-submit" title="Search">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                            <circle cx="11" cy="11" r="7"/>
                            <line x1="16.5" y1="16.5" x2="21" y2="21"/>
                        </svg>
                    </button>
                </div>
            </div>

            <!-- Quick Bang Chips (Chrome & Firefox inspired) -->
            <div class="search-chips-row">
                <span class="search-chip" onclick="quickFillBang('!g ')">🔍 Google (!g)</span>
                <span class="search-chip" onclick="quickFillBang('!ddg ')">🦆 DuckDuckGo (!ddg)</span>
                <span class="search-chip" onclick="quickFillBang('!b ')">⚡ Bing (!b)</span>
                <span class="search-chip" onclick="quickFillBang('!ahmia ')">🧅 Tor Search (!ahmia)</span>
                <span class="search-chip" onclick="quickFillBang('!w ')">📚 Wikipedia (!w)</span>
                <span class="search-chip" onclick="quickFillBang('!gh ')">🐙 GitHub (!gh)</span>
            </div>
        </div>

        <!-- Shortcuts / Speed Dial Row (Direct match to Image 1) -->
        <div class="shortcuts-section" id="shortcuts-section">
            <div class="shortcuts-row" id="shortcuts-container">
                <!-- 1. Amazon -->
                <div class="shortcut-item" data-url="https://www.amazon.com">
                    <div class="shortcut-tile" style="background-color: #111111;">
                        <svg width="26" height="26" viewBox="0 0 24 24" fill="#ff9900">
                            <path d="M12.5 15.5c-3.1 0-5.7-1.7-6.9-4.2-.2-.4.1-.9.5-1 .4-.1.9.1 1.1.5 1 2 3.1 3.2 5.3 3.2 2.6 0 4.9-1.5 5.8-3.8.2-.4.6-.6 1.1-.5.4.2.6.6.5 1.1-1.1 2.8-3.9 4.7-7.4 4.7zm6.7-2.6c-.3-.4-1-.4-1.3-.1l-1.4 1.4 1.9.6c.3.1.7 0 .9-.3.2-.4.2-1.1-.1-1.6z"/>
                            <path d="M14.5 5.5c-.8 0-1.6.4-2.1 1.1-.3-.6-.9-1.1-1.7-1.1-1.3 0-2.2 1.2-2.2 2.8v3.2h1.5V8.5c0-.8.4-1.4 1.1-1.4.6 0 1 .5 1 1.2v3.2h1.5V8.5c0-.8.4-1.4 1.1-1.4.6 0 1 .5 1 1.2v3.2h1.5V8.2c0-1.6-.9-2.7-2.7-2.7z" fill="#ffffff"/>
                        </svg>
                    </div>
                    <span class="shortcut-label">Amazon</span>
                    <span class="shortcut-sub">Sponsored</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 2. Trip.com -->
                <div class="shortcut-item" data-url="https://www.trip.com">
                    <div class="shortcut-tile" style="background-color: #1075e3; color: #ffffff; font-weight: 800; font-size: 15px;">
                        Trip.
                    </div>
                    <span class="shortcut-label">Trip.com</span>
                    <span class="shortcut-sub">Sponsored</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 3. Wikipedia -->
                <div class="shortcut-item" data-url="https://www.wikipedia.org">
                    <div class="shortcut-tile" style="background-color: #eaecf0; color: #000000; font-family: serif; font-weight: 700; font-size: 24px;">
                        W
                    </div>
                    <span class="shortcut-label">Wikipedia</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 4. YouTube -->
                <div class="shortcut-item" data-url="https://www.youtube.com">
                    <div class="shortcut-tile" style="background-color: #212121;">
                        <svg width="26" height="26" viewBox="0 0 24 24" fill="#ff0000">
                            <path d="M21.6 7.2c-.2-.9-.9-1.6-1.8-1.8C18.2 5 12 5 12 5s-6.2 0-7.8.4c-.9.2-1.6.9-1.8 1.8C2 8.8 2 12 2 12s0 3.2.4 4.8c.2.9.9 1.6 1.8 1.8 1.6.4 7.8.4 7.8.4s6.2 0 7.8-.4c.9-.2 1.6-.9 1.8-1.8.4-1.6.4-4.8.4-4.8s0-3.2-.4-4.8z"/>
                            <polygon points="9.75 15.02 15.5 12 9.75 8.98 9.75 15.02" fill="#ffffff"/>
                        </svg>
                    </div>
                    <span class="shortcut-label">YouTube</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 5. NDTV -->
                <div class="shortcut-item" data-url="https://www.ndtv.com">
                    <div class="shortcut-tile" style="background-color: #e21b22; color: #ffffff; font-weight: 900; font-size: 13px; letter-spacing: -0.5px;">
                        NDTV
                    </div>
                    <span class="shortcut-label">NDTV</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 6. Reddit -->
                <div class="shortcut-item" data-url="https://www.reddit.com">
                    <div class="shortcut-tile" style="background-color: #ff4500;">
                        <svg width="26" height="26" viewBox="0 0 24 24" fill="#ffffff">
                            <circle cx="12" cy="12" r="9"/>
                            <circle cx="9" cy="11.5" r="1.3" fill="#ff4500"/>
                            <circle cx="15" cy="11.5" r="1.3" fill="#ff4500"/>
                            <path d="M9.5 15.2c1.4.8 3.6.8 5 0" stroke="#ff4500" stroke-width="1.3" fill="none" stroke-linecap="round"/>
                        </svg>
                    </div>
                    <span class="shortcut-label">Reddit</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 7. DuckDuckGo -->
                <div class="shortcut-item" data-url="https://duckduckgo.com">
                    <div class="shortcut-tile" style="background-color: #de5833;">
                        <span style="font-size: 22px;">🦆</span>
                    </div>
                    <span class="shortcut-label">DuckDuckGo</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 8. Ahmia Tor Search -->
                <div class="shortcut-item" data-url="https://ahmia.fi">
                    <div class="shortcut-tile" style="background-color: #3b1d5c;">
                        <span style="font-size: 22px;">🧅</span>
                    </div>
                    <span class="shortcut-label">Ahmia</span>
                    <span class="shortcut-sub">Tor Search</span>
                    <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
                </div>

                <!-- 9. Add Shortcut (+) Button -->
                <div class="shortcut-item" onclick="openAddShortcutModal()">
                    <div class="shortcut-tile" style="background-color: #9059ff; color: #ffffff; font-size: 24px; font-weight: 300;">
                        +
                    </div>
                    <span class="shortcut-label">Add item</span>
                </div>
            </div>
        </div>

        <!-- Widgets Section (Image 1: World Clock, Weather, Focus Timer, Checklist) -->
        <div class="widgets-section" id="widgets-section">
            <div class="section-header">
                <div class="section-title" onclick="toggleWidgetsAccordion()">
                    <span>Widgets</span>
                    <svg id="widgets-chevron" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                        <polyline points="6 9 12 15 18 9"/>
                    </svg>
                </div>
                <button class="section-menu-btn" onclick="toggleSectionPopover('widgets-popover')">•••</button>
                <div class="section-popover" id="widgets-popover">
                    <div class="popover-item" onclick="openCustomizer('widgets')">⚙ Configure Widgets...</div>
                    <div class="popover-item" onclick="refreshClocksAndWeather()">🔄 Refresh Clocks & Weather</div>
                    <div class="popover-item" onclick="toggleWidgetsVisibility(false)">👁 Hide Widgets Section</div>
                </div>
            </div>

            <div class="widgets-grid" id="widgets-grid-content">
                <!-- 1. World Clock Card -->
                <div class="widget-card">
                    <div class="clock-grid">
                        <div class="clock-col" onclick="toggleClockFormat()">
                            <div>
                                <div class="clock-city">CCU</div>
                                <div class="clock-tz">GMT+5:30</div>
                            </div>
                            <div class="clock-time" id="clock-ccu">10:37 AM</div>
                        </div>
                        <div class="clock-col" onclick="toggleClockFormat()">
                            <div>
                                <div class="clock-city">BER</div>
                                <div class="clock-tz">GMT+2</div>
                            </div>
                            <div class="clock-time" id="clock-ber">7:07 AM</div>
                        </div>
                        <div class="clock-col" onclick="toggleClockFormat()">
                            <div>
                                <div class="clock-city">SYD</div>
                                <div class="clock-tz">GMT+10</div>
                            </div>
                            <div class="clock-time" id="clock-syd">3:07 PM</div>
                        </div>
                        <div class="clock-col" onclick="toggleClockFormat()">
                            <div>
                                <div class="clock-city">NYC</div>
                                <div class="clock-tz">EDT</div>
                            </div>
                            <div class="clock-time" id="clock-nyc">1:07 AM</div>
                        </div>
                    </div>
                </div>

                <!-- 2. Weather Card (Configurable City & °C/°F) -->
                <div class="widget-card">
                    <div class="weather-top">
                        <span class="weather-loc" id="weather-city-label" onclick="promptChangeWeatherCity()" title="Click to change city">Mumbai ✎</span>
                        <span style="color: #8f8f9d; font-size: 13px; cursor: pointer;" onclick="toggleWeatherUnit()">°C/°F</span>
                    </div>
                    <div class="weather-main">
                        <svg width="28" height="28" viewBox="0 0 24 24" fill="#f59e0b">
                            <circle cx="12" cy="12" r="5"/>
                            <line x1="12" y1="1" x2="12" y2="3" stroke="#f59e0b" stroke-width="2"/>
                            <line x1="12" y1="21" x2="12" y2="23" stroke="#f59e0b" stroke-width="2"/>
                            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" stroke="#f59e0b" stroke-width="2"/>
                            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" stroke="#f59e0b" stroke-width="2"/>
                            <line x1="1" y1="12" x2="3" y2="12" stroke="#f59e0b" stroke-width="2"/>
                            <line x1="21" y1="12" x2="23" y2="12" stroke="#f59e0b" stroke-width="2"/>
                        </svg>
                        <div>
                            <div class="weather-temp" id="weather-temp-display">31°C <small style="font-size: 11px; font-weight: normal; color: #8f8f9d;">↑36° ↓27°</small></div>
                            <div class="weather-cond" id="weather-cond-display">Mostly sunny</div>
                        </div>
                    </div>
                    <div class="weather-forecast">
                        <span>32° 11:00</span>
                        <span>34° 12:00</span>
                    </div>
                </div>

                <!-- 3. Focus / Pomodoro Timer Card (25:00 Focus/Break) -->
                <div class="widget-card">
                    <div class="timer-row">
                        <button class="timer-play-btn" id="timer-toggle-btn" onclick="toggleTimer()">▶</button>
                        <div class="timer-digits">
                            <button class="timer-ctrl-btn" onclick="adjustTimer(-60)">−</button>
                            <span id="timer-display">25:00</span>
                            <button class="timer-ctrl-btn" onclick="adjustTimer(60)">+</button>
                        </div>
                    </div>
                    <div class="timer-pills">
                        <button class="timer-pill active" id="btn-focus" onclick="setTimerMode('focus')">Focus</button>
                        <button class="timer-pill" id="btn-break" onclick="setTimerMode('break')">Break</button>
                    </div>
                </div>

                <!-- 4. Checklist Card (Interactive Tasks) -->
                <div class="widget-card">
                    <div class="checklist-title">Checklist</div>
                    <div>
                        <button class="checklist-add-btn" onclick="addChecklistItem()">
                            <span>+</span> <span>Add an item</span>
                        </button>
                    </div>
                    <div class="checklist-items-box" id="checklist-items">
                        <!-- Populated by JS -->
                    </div>
                    <div class="checklist-icon-decor">
                        <svg width="28" height="28" viewBox="0 0 24 24" fill="none">
                            <rect x="3" y="3" width="18" height="18" rx="4" fill="#38383d"/>
                            <path d="M7 8h10M7 12h6M7 16h4" stroke="#8f8f9d" stroke-width="1.8" stroke-linecap="round"/>
                            <path d="M15 14l2 2 4-4" stroke="#ff9900" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </div>
                </div>
            </div>
        </div>

        <!-- Thought-provoking stories (News Section with REAL High-Res Photography) -->
        <div class="stories-section" id="stories-section">
            <div class="section-header">
                <span class="section-title">Thought-provoking stories</span>
                <button class="section-menu-btn" onclick="toggleSectionPopover('stories-popover')">•••</button>
                <div class="section-popover" id="stories-popover">
                    <div class="popover-item" onclick="openCustomizer('news')">⚙ Personalize Content Interests...</div>
                    <div class="popover-item" onclick="refreshStoriesFeed()">🔄 Refresh Stories</div>
                    <div class="popover-item" onclick="toggleStoriesVisibility(false)">👁 Hide Stories Section</div>
                </div>
            </div>

            <!-- Content Topic Chips -->
            <div class="stories-category-chips">
                <span class="category-chip active" onclick="filterStoriesCategory('all')">All Stories</span>
                <span class="category-chip" onclick="filterStoriesCategory('tech')">Technology & AI</span>
                <span class="category-chip" onclick="filterStoriesCategory('world')">World Affairs</span>
                <span class="category-chip" onclick="filterStoriesCategory('nature')">Science & Environment</span>
                <span class="category-chip" onclick="filterStoriesCategory('culture')">Entertainment & Culture</span>
            </div>

            <div class="stories-grid" id="stories-grid-container">
                <!-- Story 1: Drishyam 3 / Ajay Devgn (Real Film Still Photography) -->
                <div class="story-card" data-category="culture" data-url="https://indianexpress.com/article/entertainment/malayalam/mohanlal-ajay-devgn-drishyam-3/">
                    <div class="story-thumb-wrap">
                        <img class="story-thumb" src="https://images.unsplash.com/photo-1536440136628-849c177e76a1?w=600&auto=format&fit=crop&q=80" 
                            alt="Drishyam 3 Film Production" 
                            onerror="this.onerror=null; this.src='https://picsum.photos/seed/drishyam3/600/320';" />
                        <span class="story-badge-corner" style="background: #e21b22;">EXPRESS</span>
                    </div>
                    <div class="story-content">
                        <h4 class="story-title">'I called Mohanlal to watch Drishyam 3': Ajay Devgn seeks Malayalam superstar's 'approval'</h4>
                        <div class="story-footer">
                            <div class="story-source">
                                <span>📰</span> <span>The Indian Express</span>
                            </div>
                            <span>Entertainment</span>
                        </div>
                    </div>
                </div>

                <!-- Story 2: The AI telling farmers when to harvest (Real Agricultural Photography) -->
                <div class="story-card" data-category="tech" data-url="https://www.bbc.com/news/technology">
                    <div class="story-thumb-wrap">
                        <img class="story-thumb" src="https://images.unsplash.com/photo-1498557850523-fd3d118b962e?w=600&auto=format&fit=crop&q=80" 
                            alt="Fresh berries harvest in orchard" 
                            onerror="this.onerror=null; this.src='https://picsum.photos/seed/harvest/600/320';" />
                        <span class="story-badge-corner" style="background: #000000;">BBC NEWS</span>
                    </div>
                    <div class="story-content">
                        <h4 class="story-title">The AI telling farmers when to harvest</h4>
                        <div class="story-footer">
                            <div class="story-source">
                                <span style="background: #000; color: #fff; padding: 1px 4px; border-radius: 2px; font-weight: bold; font-size: 9px;">BBC</span>
                                <span>BBC News</span>
                            </div>
                            <span>Technology</span>
                        </div>
                    </div>
                </div>

                <!-- Story 3: AI used to keep elephants out of habitations on Sabarimala (Real Wildlife Photography) -->
                <div class="story-card" data-category="nature" data-url="https://www.thehindu.com">
                    <div class="story-thumb-wrap">
                        <img class="story-thumb" src="https://images.unsplash.com/photo-1557050543-4d5f4e07ef46?w=600&auto=format&fit=crop&q=80" 
                            alt="Wild elephant in sanctuary forest" 
                            onerror="this.onerror=null; this.src='https://picsum.photos/seed/elephant/600/320';" />
                        <span class="story-badge-corner" style="background: #1a365d;">ENVIRONMENT</span>
                    </div>
                    <div class="story-content">
                        <h4 class="story-title">AI used to keep elephants out of habitations on Sabarimala forest fringes</h4>
                        <div class="story-footer">
                            <div class="story-source">
                                <span>🏛️</span> <span>The Hindu</span>
                            </div>
                            <span>Science</span>
                        </div>
                    </div>
                </div>

                <!-- Story 4: Flydubai Incident Live Updates (Real Commercial Aviation Photography) -->
                <div class="story-card" data-category="world" data-url="https://www.news18.com">
                    <div class="story-thumb-wrap">
                        <img class="story-thumb" src="https://images.unsplash.com/photo-1540959733332-eab4deabeeaf?w=600&auto=format&fit=crop&q=80" 
                            alt="Passenger airliner flying under clear sky" 
                            onerror="this.onerror=null; this.src='https://picsum.photos/seed/aviation/600/320';" />
                        <span class="story-badge-corner" style="background: #c53030;">NEWS18</span>
                    </div>
                    <div class="story-content">
                        <h4 class="story-title">Flydubai Incident Live Updates: Netanyahu Says Indian Pilot Averted '91-Style' Attack</h4>
                        <div class="story-footer">
                            <div class="story-source">
                                <span style="color: #e53e3e; font-weight: bold;">NEWS18</span>
                            </div>
                            <span>World News</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Privacy Shields & Internal Command Quick Access -->
        <div class="vault-pills-bar" id="vaults-bar">
            <span style="font-size: 12px; color: #8f8f9d; margin-right: 4px;">NORVA Vaults:</span>
            <a href="norva://history" class="vault-pill-link">📜 History</a>
            <a href="norva://shields" class="vault-pill-link">🛡️ Shields ({{ADS_BLOCKED}} blocked)</a>
            <a href="norva://permissions" class="vault-pill-link">📷 Permissions</a>
            <a href="norva://cookies" class="vault-pill-link">🍪 Storage ({{CACHE_MB}} MB)</a>
            <a href="norva://settings" class="vault-pill-link">⚙️ Settings</a>
        </div>
    </div>

    <!-- Floating Customise Page Button (Direct match to Chrome Image 2 & Firefox Image 1) -->
    <button class="customize-floating-pill" id="customize-btn" onclick="openCustomizer()">
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M12 20h9"/>
            <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/>
        </svg>
        <span>Customise page</span>
    </button>

    <!-- Comprehensive Homepage Customizer Drawer -->
    <div class="customizer-drawer-overlay" id="customizer-overlay" onclick="closeCustomizer()"></div>
    <div class="customizer-drawer" id="customizer-drawer">
        <div class="drawer-header">
            <span class="drawer-title">Customise this page</span>
            <button class="drawer-close-btn" onclick="closeCustomizer()">✕</button>
        </div>
        <div class="drawer-body">
            <!-- 1. Background Theme Selector -->
            <div class="drawer-section">
                <div class="drawer-section-title">Appearance & Wallpaper</div>
                <div class="theme-swatches-grid">
                    <div class="theme-swatch active" style="background-color: #1c1b22;" onclick="setThemeBackground('#1c1b22', this)">
                        Firefox
                    </div>
                    <div class="theme-swatch" style="background-color: #202124;" onclick="setThemeBackground('#202124', this)">
                        Chrome
                    </div>
                    <div class="theme-swatch" style="background-color: #0d0d0f;" onclick="setThemeBackground('#0d0d0f', this)">
                        Midnight
                    </div>
                    <div class="theme-swatch" style="background: linear-gradient(135deg, #181928, #0e1017);" onclick="setThemeBackground('linear-gradient(135deg, #181928, #0e1017)', this)">
                        Indigo
                    </div>
                    <div class="theme-swatch" style="background: linear-gradient(135deg, #112217, #0b140f);" onclick="setThemeBackground('linear-gradient(135deg, #112217, #0b140f)', this)">
                        Forest
                    </div>
                    <div class="theme-swatch" style="background: radial-gradient(circle at 50% 0%, #2b1f42, #161722);" onclick="setThemeBackground('radial-gradient(circle at 50% 0%, #2b1f42, #161722)', this)">
                        Aurora
                    </div>
                </div>
            </div>

            <!-- 2. Shortcuts Options -->
            <div class="drawer-section">
                <div class="drawer-section-title">Shortcuts</div>
                <div class="drawer-toggle-row">
                    <span>Show shortcuts</span>
                    <label class="toggle-switch">
                        <input type="checkbox" id="toggle-shortcuts" checked onchange="toggleShortcutsVisibility(this.checked)" />
                        <span class="slider"></span>
                    </label>
                </div>
                <div class="drawer-toggle-row">
                    <span>Sponsored shortcuts</span>
                    <label class="toggle-switch">
                        <input type="checkbox" id="toggle-sponsored" checked onchange="toggleSponsoredShortcuts(this.checked)" />
                        <span class="slider"></span>
                    </label>
                </div>
                <button class="btn btn-primary" style="margin-top: 10px; width: 100%; justify-content: center;" onclick="openAddShortcutModal()">
                    + Add New Shortcut
                </button>
            </div>

            <!-- 3. Widgets Configuration -->
            <div class="drawer-section" id="drawer-widgets-sec">
                <div class="drawer-section-title">Widgets & Tools</div>
                <div class="drawer-toggle-row">
                    <span>Show widgets</span>
                    <label class="toggle-switch">
                        <input type="checkbox" id="toggle-widgets" checked onchange="toggleWidgetsVisibility(this.checked)" />
                        <span class="slider"></span>
                    </label>
                </div>
                <div style="margin-top: 10px;">
                    <label style="font-size: 12px; color: #cfcfd4;">Weather Location:</label>
                    <input type="text" id="weather-city-input" class="modal-input" value="Mumbai" onchange="setWeatherCity(this.value)" style="margin-top: 6px; padding: 6px 10px;" />
                </div>
            </div>

            <!-- 4. Stories Configuration -->
            <div class="drawer-section" id="drawer-news-sec">
                <div class="drawer-section-title">Thought-Provoking Stories</div>
                <div class="drawer-toggle-row">
                    <span>Show stories feed</span>
                    <label class="toggle-switch">
                        <input type="checkbox" id="toggle-stories" checked onchange="toggleStoriesVisibility(this.checked)" />
                        <span class="slider"></span>
                    </label>
                </div>
                <div class="drawer-toggle-row">
                    <span>Show Privacy Vaults strip</span>
                    <label class="toggle-switch">
                        <input type="checkbox" id="toggle-vaults" checked onchange="toggleVaultsVisibility(this.checked)" />
                        <span class="slider"></span>
                    </label>
                </div>
            </div>
        </div>
    </div>

    <!-- Add Shortcut Modal -->
    <div class="modal-overlay" id="add-shortcut-modal">
        <div class="modal-box">
            <div class="modal-title">Add shortcut</div>
            <label style="font-size: 12px; color: #cfcfd4; display: block; margin-bottom: 4px;">Name:</label>
            <input type="text" id="modal-name-input" class="modal-input" placeholder="e.g. GitHub" />
            <label style="font-size: 12px; color: #cfcfd4; display: block; margin-bottom: 4px;">URL:</label>
            <input type="text" id="modal-url-input" class="modal-input" placeholder="https://github.com" />
            <div class="modal-actions">
                <button class="btn" onclick="closeAddShortcutModal()">Cancel</button>
                <button class="btn btn-primary" onclick="confirmAddShortcut()">Done</button>
            </div>
        </div>
    </div>

    <script>
        // --------------------------------------------------------------------
        // Search & Engine Switcher
        // --------------------------------------------------------------------
        let activeEngineId = "{{DEFAULT_ID}}";
        let activeEngineBang = "{{DEFAULT_BANG}}";
        const searchInput = document.getElementById("search-input");
        const searchSubmitBtn = document.getElementById("search-submit");
        const searchClearBtn = document.getElementById("search-clear");
        const engineDropdown = document.getElementById("engine-dropdown");

        function toggleEngineDropdown() {
            engineDropdown.classList.toggle("show");
        }

        window.addEventListener("click", (e) => {
            if (!e.target.closest("#engine-picker-trigger") && !e.target.closest("#engine-dropdown")) {
                engineDropdown.classList.remove("show");
            }
            if (!e.target.closest(".section-menu-btn") && !e.target.closest(".section-popover")) {
                document.querySelectorAll(".section-popover").forEach(p => p.classList.remove("show"));
            }
        });

        function selectSearchEngine(id, name, bang, icon) {
            activeEngineId = id;
            activeEngineBang = bang;
            document.getElementById("active-engine-icon").textContent = icon;
            document.getElementById("active-engine-label").textContent = name;
            searchInput.placeholder = "Search with " + name + " or enter address";
            document.querySelectorAll(".engine-dropdown-item").forEach(item => {
                item.classList.toggle("active", item.getAttribute("data-id") === id);
            });
            engineDropdown.classList.remove("show");
            searchInput.focus();
        }

        searchInput.addEventListener("input", () => {
            searchClearBtn.style.display = searchInput.value.length > 0 ? "block" : "none";
        });

        function clearSearchInput() {
            searchInput.value = "";
            searchClearBtn.style.display = "none";
            searchInput.focus();
        }

        function quickFillBang(bang) {
            searchInput.value = bang;
            searchClearBtn.style.display = "block";
            searchInput.focus();
        }

        function executeSearch() {
            const val = searchInput.value.trim();
            if (!val) return;

            let query = val;
            if (activeEngineBang && !val.startsWith("!") && !val.includes("://") && !val.includes(".")) {
                query = activeEngineBang + " " + val;
            }

            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "navigate", query: query }));
            }
        }

        searchSubmitBtn.addEventListener("click", executeSearch);
        searchInput.addEventListener("keydown", (e) => {
            if (e.key === "Enter") executeSearch();
        });

        // --------------------------------------------------------------------
        // Shortcuts Management & Card Navigation
        // --------------------------------------------------------------------
        function bindShortcutClicks() {
            document.querySelectorAll(".shortcut-item, .story-card, .vault-pill-link").forEach(card => {
                card.onclick = (e) => {
                    const url = card.getAttribute("data-url") || card.getAttribute("href");
                    if (url && url !== "#") {
                        e.preventDefault();
                        if (window.ipc && window.ipc.postMessage) {
                            window.ipc.postMessage(JSON.stringify({ action: "navigate", query: url }));
                        }
                    }
                };
            });
        }
        bindShortcutClicks();

        function removeShortcut(btn) {
            const item = btn.closest(".shortcut-item");
            if (item) item.remove();
        }

        function openAddShortcutModal() {
            document.getElementById("add-shortcut-modal").classList.add("show");
            document.getElementById("modal-name-input").focus();
        }

        function closeAddShortcutModal() {
            document.getElementById("add-shortcut-modal").classList.remove("show");
            document.getElementById("modal-name-input").value = "";
            document.getElementById("modal-url-input").value = "";
        }

        function confirmAddShortcut() {
            const name = document.getElementById("modal-name-input").value.trim();
            let url = document.getElementById("modal-url-input").value.trim();
            if (!name || !url) {
                alert("Please enter both Name and URL.");
                return;
            }
            if (!url.startsWith("http://") && !url.startsWith("https://") && !url.startsWith("norva://")) {
                url = "https://" + url;
            }

            const initial = name.charAt(0).toUpperCase();
            const container = document.getElementById("shortcuts-container");
            const plusBtn = container.lastElementChild;

            const div = document.createElement("div");
            div.className = "shortcut-item";
            div.setAttribute("data-url", url);
            div.innerHTML = `
                <div class="shortcut-tile" style="background-color: #38383d; color: #ffffff; font-weight: 700; font-size: 20px;">
                    ${initial}
                </div>
                <span class="shortcut-label">${name}</span>
                <div class="shortcut-menu-trigger" onclick="event.stopPropagation(); removeShortcut(this)">✕</div>
            `;
            container.insertBefore(div, plusBtn);
            bindShortcutClicks();
            closeAddShortcutModal();
        }

        // --------------------------------------------------------------------
        // Section Popovers
        // --------------------------------------------------------------------
        function toggleSectionPopover(id) {
            const pop = document.getElementById(id);
            if (pop) pop.classList.toggle("show");
        }

        function toggleWidgetsAccordion() {
            const grid = document.getElementById("widgets-grid-content");
            const chev = document.getElementById("widgets-chevron");
            if (grid) {
                const hidden = grid.style.display === "none";
                grid.style.display = hidden ? "grid" : "none";
                chev.style.transform = hidden ? "rotate(0deg)" : "rotate(-90deg)";
            }
        }

        // --------------------------------------------------------------------
        // World Clock (Live Time Updates)
        // --------------------------------------------------------------------
        let use24HourClock = false;
        function toggleClockFormat() {
            use24HourClock = !use24HourClock;
            updateWorldClocks();
        }

        function updateWorldClocks() {
            const now = new Date();
            const formatTime = (offsetHours) => {
                const utc = now.getTime() + (now.getTimezoneOffset() * 60000);
                const target = new Date(utc + (3600000 * offsetHours));
                return target.toLocaleTimeString([], { 
                    hour: 'numeric', 
                    minute: '2-digit', 
                    hour12: !use24HourClock 
                });
            };

            const elCcu = document.getElementById("clock-ccu");
            const elBer = document.getElementById("clock-ber");
            const elSyd = document.getElementById("clock-syd");
            const elNyc = document.getElementById("clock-nyc");

            if (elCcu) elCcu.textContent = formatTime(5.5);
            if (elBer) elBer.textContent = formatTime(2.0);
            if (elSyd) elSyd.textContent = formatTime(10.0);
            if (elNyc) elNyc.textContent = formatTime(-4.0);
        }
        updateWorldClocks();
        setInterval(updateWorldClocks, 1000);

        // --------------------------------------------------------------------
        // Weather Widget Settings & Controls
        // --------------------------------------------------------------------
        let isFahrenheit = false;
        let weatherCity = "Mumbai";
        let baseTempC = 31;

        function updateWeatherDisplay() {
            const tempDisplay = document.getElementById("weather-temp-display");
            const cityLabel = document.getElementById("weather-city-label");
            if (cityLabel) cityLabel.textContent = weatherCity + " ✎";

            if (tempDisplay) {
                if (isFahrenheit) {
                    const tempF = Math.round((baseTempC * 9/5) + 32);
                    tempDisplay.innerHTML = `${tempF}°F <small style="font-size: 11px; font-weight: normal; color: #8f8f9d;">↑97° ↓81°</small>`;
                } else {
                    tempDisplay.innerHTML = `${baseTempC}°C <small style="font-size: 11px; font-weight: normal; color: #8f8f9d;">↑36° ↓27°</small>`;
                }
            }
        }

        function toggleWeatherUnit() {
            isFahrenheit = !isFahrenheit;
            updateWeatherDisplay();
        }

        function promptChangeWeatherCity() {
            const city = prompt("Enter city name for Weather:", weatherCity);
            if (city && city.trim()) {
                setWeatherCity(city.trim());
            }
        }

        function setWeatherCity(city) {
            weatherCity = city;
            // Preset realistic temperatures for common cities
            const cLower = city.toLowerCase();
            if (cLower.includes("london")) baseTempC = 17;
            else if (cLower.includes("new york") || cLower.includes("nyc")) baseTempC = 19;
            else if (cLower.includes("tokyo")) baseTempC = 22;
            else if (cLower.includes("sydney")) baseTempC = 20;
            else if (cLower.includes("delhi")) baseTempC = 33;
            else baseTempC = 28;

            updateWeatherDisplay();
            const input = document.getElementById("weather-city-input");
            if (input) input.value = weatherCity;
        }

        function refreshClocksAndWeather() {
            updateWorldClocks();
            updateWeatherDisplay();
            document.querySelectorAll(".section-popover").forEach(p => p.classList.remove("show"));
        }

        // --------------------------------------------------------------------
        // Focus Timer Logic
        // --------------------------------------------------------------------
        let timerSeconds = 25 * 60;
        let timerInterval = null;
        let timerRunning = false;

        function updateTimerDisplay() {
            const m = Math.floor(timerSeconds / 60);
            const s = timerSeconds % 60;
            const str = String(m).padStart(2, '0') + ":" + String(s).padStart(2, '0');
            const el = document.getElementById("timer-display");
            if (el) el.textContent = str;
        }

        function toggleTimer() {
            const btn = document.getElementById("timer-toggle-btn");
            if (timerRunning) {
                clearInterval(timerInterval);
                timerRunning = false;
                if (btn) btn.textContent = "▶";
            } else {
                timerRunning = true;
                if (btn) btn.textContent = "❚❚";
                timerInterval = setInterval(() => {
                    if (timerSeconds > 0) {
                        timerSeconds--;
                        updateTimerDisplay();
                    } else {
                        clearInterval(timerInterval);
                        timerRunning = false;
                        if (btn) btn.textContent = "▶";
                        alert("Focus session complete!");
                    }
                }, 1000);
            }
        }

        function adjustTimer(delta) {
            timerSeconds = Math.max(60, timerSeconds + delta);
            updateTimerDisplay();
        }

        function setTimerMode(mode) {
            clearInterval(timerInterval);
            timerRunning = false;
            const btn = document.getElementById("timer-toggle-btn");
            if (btn) btn.textContent = "▶";

            document.getElementById("btn-focus").classList.toggle("active", mode === "focus");
            document.getElementById("btn-break").classList.toggle("active", mode === "break");

            timerSeconds = mode === "focus" ? 25 * 60 : 5 * 60;
            updateTimerDisplay();
        }

        // --------------------------------------------------------------------
        // Checklist Logic
        // --------------------------------------------------------------------
        let checklistData = ["Research Rust WebKitGTK", "Read tech privacy news"];
        function renderChecklist() {
            const container = document.getElementById("checklist-items");
            if (!container) return;
            container.innerHTML = "";
            checklistData.forEach((task, idx) => {
                const row = document.createElement("div");
                row.className = "checklist-item-row";
                row.innerHTML = `
                    <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
                        <input type="checkbox" onchange="this.parentElement.style.textDecoration = this.checked ? 'line-through' : 'none'" />
                        <span>${task}</span>
                    </label>
                    <span style="color: #8f8f9d; cursor: pointer; padding: 0 4px;" onclick="removeChecklistItem(${idx})">✕</span>
                `;
                container.appendChild(row);
            });
        }
        renderChecklist();

        function addChecklistItem() {
            const item = prompt("Add a task to Checklist:");
            if (item && item.trim()) {
                checklistData.push(item.trim());
                renderChecklist();
            }
        }

        function removeChecklistItem(idx) {
            checklistData.splice(idx, 1);
            renderChecklist();
        }

        // --------------------------------------------------------------------
        // Stories / News Category Filter
        // --------------------------------------------------------------------
        function filterStoriesCategory(cat) {
            document.querySelectorAll(".category-chip").forEach(chip => chip.classList.remove("active"));
            event.target.classList.add("active");

            const cards = document.querySelectorAll(".story-card");
            cards.forEach(card => {
                const cardCat = card.getAttribute("data-category");
                if (cat === "all" || cardCat === cat) {
                    card.style.display = "flex";
                } else {
                    card.style.display = "none";
                }
            });
        }

        function refreshStoriesFeed() {
            document.querySelectorAll(".section-popover").forEach(p => p.classList.remove("show"));
            const cards = document.querySelectorAll(".story-card");
            cards.forEach(c => {
                c.style.opacity = "0.4";
                setTimeout(() => c.style.opacity = "1", 200);
            });
        }

        // --------------------------------------------------------------------
        // Homepage Customizer Drawer System
        // --------------------------------------------------------------------
        function openCustomizer(focusSec) {
            document.getElementById("customizer-overlay").style.display = "block";
            document.getElementById("customizer-drawer").classList.add("open");
            if (focusSec === "widgets") {
                document.getElementById("drawer-widgets-sec").scrollIntoView({ behavior: 'smooth' });
            } else if (focusSec === "news") {
                document.getElementById("drawer-news-sec").scrollIntoView({ behavior: 'smooth' });
            }
        }

        function closeCustomizer() {
            document.getElementById("customizer-overlay").style.display = "none";
            document.getElementById("customizer-drawer").classList.remove("open");
        }

        function setThemeBackground(bg, elem) {
            document.body.style.background = bg;
            document.querySelectorAll(".theme-swatch").forEach(s => s.classList.remove("active"));
            if (elem) elem.classList.add("active");
            try { localStorage.setItem("norva_theme_bg", bg); } catch(e) {}
        }

        // Restore saved theme background
        try {
            const savedBg = localStorage.getItem("norva_theme_bg");
            if (savedBg) document.body.style.background = savedBg;
        } catch(e) {}

        function toggleShortcutsVisibility(show) {
            const el = document.getElementById("shortcuts-section");
            if (el) el.style.display = show ? "block" : "none";
        }

        function toggleSponsoredShortcuts(show) {
            document.querySelectorAll(".shortcut-sub").forEach(sub => {
                if (sub.textContent.includes("Sponsored")) {
                    sub.closest(".shortcut-item").style.display = show ? "flex" : "none";
                }
            });
        }

        function toggleWidgetsVisibility(show) {
            const el = document.getElementById("widgets-section");
            if (el) el.style.display = show ? "block" : "none";
            document.getElementById("toggle-widgets").checked = show;
        }

        function toggleStoriesVisibility(show) {
            const el = document.getElementById("stories-section");
            if (el) el.style.display = show ? "block" : "none";
            document.getElementById("toggle-stories").checked = show;
        }

        function toggleVaultsVisibility(show) {
            const el = document.getElementById("vaults-bar");
            if (el) el.style.display = show ? "flex" : "none";
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{MODE_PILL}}", mode_pill)
        .replace("{{DEFAULT_NAME}}", &default_engine.name)
        .replace("{{DEFAULT_ICON}}", default_icon)
        .replace("{{DEFAULT_BANG}}", &default_engine.bang)
        .replace("{{DEFAULT_ID}}", &default_engine.id)
        .replace("{{ENGINE_ITEMS}}", &engine_items_html)
        .replace("{{ADS_BLOCKED}}", &stats.total_ads_blocked.to_string())
        .replace("{{CACHE_MB}}", &format!("{:.1}", cache_mb))
}

/// Renders the `norva://history` view.
pub fn render_history_page(entries: &[HistoryEntry]) -> String {
    let mut rows = String::new();
    if entries.is_empty() {
        rows.push_str(r###"<div style="text-align:center; padding: 48px; color: var(--text-muted);">
            <h3>Your History Vault is Empty</h3>
            <p style="margin-top: 8px;">No browsing activity recorded. In Secure Mode, history is never saved.</p>
        </div>"###);
    } else {
        for e in entries {
            let engine_badge = if let Some(ref eng) = e.search_engine {
                format!(r###"<span class="badge-engine">🔍 {}</span>"###, eng)
            } else {
                String::new()
            };

            let query_info = if let Some(ref q) = e.search_query {
                format!(r###"<span style="color: var(--accent-blue); font-weight: 500;">Search: "{}"</span> - "###, q)
            } else {
                String::new()
            };

            rows.push_str(&format!(
                r###"<div class="history-item">
                    <div class="history-main">
                        <div class="history-title">
                            <a href="#" class="history-link" data-url="{}">{}</a>
                            {}
                        </div>
                        <div class="history-meta">
                            {}{} - Visited {} times
                        </div>
                    </div>
                    <button class="btn btn-danger btn-sm" onclick="deleteHistory({})" title="Delete record">Delete</button>
                </div>"###,
                e.url, e.title, engine_badge, query_info, e.visit_time, e.visit_count, e.id
            ));
        }
    }

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>History</title>
    <style>
        {{COMMON_CSS}}
        .header-actions {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 24px;
        }
        .search-box {
            background: #1c1b22;
            border: 1px solid var(--border-subtle);
            border-radius: 6px;
            padding: 8px 14px;
            color: #fbfbfe;
            width: 320px;
            outline: none;
        }
        .search-box:focus {
            border-color: var(--border-focus);
            box-shadow: 0 0 0 1px var(--border-focus);
        }
        .history-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 16px;
            background: #2b2a33;
            border: 1px solid var(--border-subtle);
            border-radius: 8px;
            margin-bottom: 8px;
            transition: background-color 0.15s ease;
        }
        .history-item:hover {
            background-color: #38383d;
        }
        .history-main {
            overflow: hidden;
        }
        .history-title {
            font-size: 14px;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 4px;
        }
        .history-link {
            color: var(--text-primary);
            text-decoration: none;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
        .history-link:hover {
            color: var(--accent-blue);
        }
        .history-meta {
            font-size: 12px;
            color: var(--text-muted);
        }
        .badge-engine {
            background-color: #42414d;
            color: #cfcfd4;
            padding: 2px 6px;
            border-radius: 4px;
            font-size: 11px;
        }
            font-size: 11px;
        }
        .btn-sm {
            padding: 4px 10px;
            font-size: 12px;
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">📜 History Vault</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip active">📜 History</a>
            <a href="norva://shields" class="chip">🛡️ Shields</a>
            <a href="norva://permissions" class="chip">📷 Permissions</a>
            <a href="norva://cookies" class="chip">🍪 Storage & Cache</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <div class="header-actions">
            <div>
                <h2>Browsing History Vault</h2>
                <p style="color: var(--text-muted); font-size: 14px;">Dedicated SQLite history.db storage</p>
            </div>
            <div style="display: flex; gap: 10px;">
                <input type="text" class="search-box" id="search-hist" placeholder="Filter history..." oninput="filterHistory()" />
                <button class="btn btn-danger" onclick="clearAllHistory()">Purge All History</button>
            </div>
        </div>

        <div class="card" id="history-container">
            {{ROWS}}
        </div>
    </main>

    <script>
        document.querySelectorAll(".history-link").forEach(link => {
            link.addEventListener("click", (e) => {
                e.preventDefault();
                const url = link.getAttribute("data-url");
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "navigate", query: url }));
                }
            });
        });

        function deleteHistory(id) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "delete_history", id: id }));
                setTimeout(() => window.location.reload(), 150);
            }
        }

        function clearAllHistory() {
            if (confirm("Permanently wipe all browsing history from history.db?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "clear_history" }));
                    setTimeout(() => window.location.reload(), 200);
                }
            }
        }

        function filterHistory() {
            const val = document.getElementById("search-hist").value.toLowerCase();
            document.querySelectorAll(".history-item").forEach(item => {
                const text = item.textContent.toLowerCase();
                item.style.display = text.includes(val) ? "flex" : "none";
            });
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{ROWS}}", &rows)
}

/// Renders the `norva://shields` view.
pub fn render_shields_page(_stats: &ShieldStats, events: &[BlockedEvent]) -> String {
    let mut log_rows = String::new();
    for ev in events {
        let badge_color = match ev.category {
            crate::db::shields_db::BlockCategory::Ad => "#38bdf8",
            crate::db::shields_db::BlockCategory::Tracker => "#f43f5e",
            crate::db::shields_db::BlockCategory::Fingerprinter => "#a855f7",
            crate::db::shields_db::BlockCategory::Cryptominer => "#eab308",
            crate::db::shields_db::BlockCategory::TrackingParam => "#10b981",
        };
        log_rows.push_str(&format!(
            r###"<tr>
                <td><span style="background: {}; color: #000; padding: 2px 6px; border-radius: 4px; font-weight: 700; font-size: 11px;">{:?}</span></td>
                <td style="font-family: monospace; color: var(--text-primary); font-weight: 500;">{}</td>
                <td>{}</td>
                <td style="color: var(--text-muted); font-size: 12px;">{}</td>
            </tr>"###,
            badge_color, ev.category, ev.blocked_domain, ev.page_domain, ev.timestamp
        ));
    }

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>NORVA | Privacy Shields & Ad-Blocker</title>
    <style>
        {{COMMON_CSS}}
        .table {
            width: 100%;
            border-collapse: collapse;
            font-size: 14px;
        }
        .table th, .table td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }
        .table th {
            color: var(--text-muted);
            text-transform: uppercase;
            font-size: 11px;
            letter-spacing: 0.8px;
        }
        .switch-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 14px 0;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }
        .switch-row:last-child {
            border-bottom: none;
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">🛡️ Privacy Shields Center</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip active">🛡️ Shields</a>
            <a href="norva://permissions" class="chip">📷 Permissions</a>
            <a href="norva://cookies" class="chip">🍪 Storage & Cache</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <h2>Privacy Shields & Anti-Tracking Command</h2>
        <p style="color: var(--text-muted); margin-bottom: 24px;">Dedicated SQLite privacy_shields.db rule engine</p>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Active Defense Matrix</h3>
            <div class="switch-row">
                <div>
                    <strong>Aggressive Network Ad Blocking</strong>
                    <p style="color: var(--text-muted); font-size: 13px;">Pre-emptively blocks doubleclick, criteo, taboola, outbrain sockets</p>
                </div>
                <span class="mode-badge normal">ENABLED</span>
            </div>
            <div class="switch-row">
                <div>
                    <strong>Behavioral Tracker & Telemetry Protection</strong>
                    <p style="color: var(--text-muted); font-size: 13px;">Blocks Google Analytics, Hotjar, Clarity, Facebook Pixel, Yandex</p>
                </div>
                <span class="mode-badge normal">ENABLED</span>
            </div>
            <div class="switch-row">
                <div>
                    <strong>Canvas & Audio Fingerprint Defense</strong>
                    <p style="color: var(--text-muted); font-size: 13px;">Injects deterministic sub-pixel noise into canvas renders</p>
                </div>
                <span class="mode-badge normal">ENABLED</span>
            </div>
            <div class="switch-row">
                <div>
                    <strong>URL Parameter Stripping</strong>
                    <p style="color: var(--text-muted); font-size: 13px;">Sanitizes fbclid, gclid, utm_source, msclkid from destination links</p>
                </div>
                <span class="mode-badge normal">ENABLED</span>
            </div>
        </div>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Recent Neutralized Surveillance Events</h3>
            <table class="table">
                <thead>
                    <tr>
                        <th>Category</th>
                        <th>Blocked Entity</th>
                        <th>Origin Page</th>
                        <th>Timestamp</th>
                    </tr>
                </thead>
                <tbody>
                    {{LOG_ROWS}}
                </tbody>
            </table>
        </div>
    </main>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{LOG_ROWS}}", &log_rows)
}

/// Renders the `norva://permissions` view.
pub fn render_permissions_page(permissions: &[SitePermission], audits: &[PermissionAuditLog]) -> String {
    let mut perm_rows = String::new();
    for p in permissions {
        perm_rows.push_str(&format!(
            r###"<tr>
                <td style="font-weight: 600;">{}</td>
                <td><span class="mode-badge {}">{:?}</span></td>
                <td><span class="mode-badge {}">{:?}</span></td>
                <td><span class="mode-badge {}">{:?}</span></td>
                <td><button class="btn btn-danger btn-sm" onclick="resetPermission('{}')">Reset</button></td>
            </tr>"###,
            p.domain,
            if p.camera == crate::db::permissions_db::PermissionPolicy::Allow { "normal" } else { "secure" },
            p.camera,
            if p.microphone == crate::db::permissions_db::PermissionPolicy::Allow { "normal" } else { "secure" },
            p.microphone,
            if p.location == crate::db::permissions_db::PermissionPolicy::Allow { "normal" } else { "secure" },
            p.location,
            p.domain
        ));
    }

    let mut audit_rows = String::new();
    for a in audits {
        audit_rows.push_str(&format!(
            r###"<tr>
                <td>{}</td>
                <td style="text-transform: capitalize;">{}</td>
                <td><span class="mode-badge {}">{}</span></td>
                <td style="color: var(--text-muted); font-size: 12px;">{}</td>
            </tr>"###,
            a.domain,
            a.permission_type,
            if a.decision.contains("allow") { "normal" } else { "secure" },
            a.decision,
            a.timestamp
        ));
    }

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>NORVA | Hardware & Location Permissions</title>
    <style>
        {{COMMON_CSS}}
        .table {
            width: 100%;
            border-collapse: collapse;
            font-size: 14px;
        }
        .table th, .table td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }
        .btn-sm {
            padding: 4px 8px;
            font-size: 11px;
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">📷 Hardware & Site Permissions</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip">🛡️ Shields</a>
            <a href="norva://permissions" class="chip active">📷 Permissions</a>
            <a href="norva://cookies" class="chip">🍪 Storage & Cache</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <h2>Hardware & Location Permissions Arbiter</h2>
        <p style="color: var(--text-muted); margin-bottom: 24px;">Dedicated SQLite permissions.db rule arbiter for Internal/External Webcams, Microphones, and GPS.</p>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Configured Domain Rules</h3>
            <table class="table">
                <thead>
                    <tr>
                        <th>Domain</th>
                        <th>📷 Camera</th>
                        <th>🎙️ Microphone</th>
                        <th>📍 Location</th>
                        <th>Action</th>
                    </tr>
                </thead>
                <tbody>
                    {{PERM_ROWS}}
                </tbody>
            </table>
        </div>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Hardware Invocation Forensic Audit Log</h3>
            <table class="table">
                <thead>
                    <tr>
                        <th>Origin</th>
                        <th>Device Requested</th>
                        <th>Resolution</th>
                        <th>Timestamp</th>
                    </tr>
                </thead>
                <tbody>
                    {{AUDIT_ROWS}}
                </tbody>
            </table>
        </div>
    </main>

    <script>
        function resetPermission(domain) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "reset_permission", domain: domain }));
                setTimeout(() => window.location.reload(), 150);
            }
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{PERM_ROWS}}", &perm_rows)
        .replace("{{AUDIT_ROWS}}", &audit_rows)
}

/// Renders the `norva://cookies` view.
pub fn render_cookies_page(cache_mb: f64) -> String {
    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>NORVA | Cookie Isolation & Cache Management</title>
    <style>
        {{COMMON_CSS}}
        .storage-stat {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 16px;
            background: rgba(15, 23, 42, 0.5);
            border-radius: 12px;
            margin-bottom: 16px;
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">🍪 Cookie Isolation & Cache Management</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip">🛡️ Shields</a>
            <a href="norva://permissions" class="chip">📷 Permissions</a>
            <a href="norva://cookies" class="chip active">🍪 Storage & Cache</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <h2>Cookie Isolation & Cache Memory Management</h2>
        <p style="color: var(--text-muted); margin-bottom: 24px;">Dedicated SQLite cache_cookies.db storage controller</p>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Cache Memory Health</h3>
            <div class="storage-stat">
                <div>
                    <h4 style="font-size: 22px; color: var(--text-primary); font-weight: 700;">{{CACHE_MB}} MB</h4>
                    <p style="color: var(--text-muted); font-size: 13px;">Physical disk cache footprint</p>
                </div>
                <button class="btn btn-danger" onclick="purgeCache()">Purge Cache Memory Now</button>
            </div>
            <p style="font-size: 13px; color: var(--text-secondary);">
                Purging cache resets all locally stored stylesheets, images, and cached responses, preventing ETag super-cookie tracking.
            </p>
        </div>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Cookie Partitioning Matrix</h3>
            <div style="display: flex; flex-direction: column; gap: 14px;">
                <div style="display: flex; justify-content: space-between; align-items: center;">
                    <div>
                        <strong>Block All Third-Party Tracking Cookies</strong>
                        <p style="color: var(--text-muted); font-size: 13px;">Denies cross-site advertising networks from dropping persistent identifiers</p>
                    </div>
                    <span class="mode-badge normal">ENFORCED</span>
                </div>
                <div style="display: flex; justify-content: space-between; align-items: center;">
                    <div>
                        <strong>Automatic Ephemeral Cookie Wipe</strong>
                        <p style="color: var(--text-muted); font-size: 13px;">Purges unauthenticated site cookies upon closing windows</p>
                    </div>
                    <span class="mode-badge normal">ENFORCED</span>
                </div>
            </div>
        </div>
    </main>

    <script>
        function purgeCache() {
            if (confirm("Purge all disk cache memory?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "purge_cache" }));
                    alert("Cache memory purged successfully!");
                    setTimeout(() => window.location.reload(), 200);
                }
            }
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{CACHE_MB}}", &format!("{:.2}", cache_mb))
}

/// Renders the `norva://settings` view.
pub fn render_settings_page(
    engines: &[SearchEngineConfig],
    default_engine: &SearchEngineConfig,
    _secure_mode: bool,
) -> String {
    let mut engine_options = String::new();
    for e in engines {
        let sel = if e.id == default_engine.id { "selected" } else { "" };
        engine_options.push_str(&format!(
            r###"<option value="{}" {}>{} ({})</option>"###,
            e.id, sel, e.name, e.bang
        ));
    }

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>NORVA | Settings & Search Engine Configuration</title>
    <style>
        {{COMMON_CSS}}
        .form-group {
            margin-bottom: 20px;
        }
        .form-group label {
            display: block;
            font-size: 14px;
            font-weight: 600;
            margin-bottom: 6px;
        }
        .select-box, .input-box {
            width: 100%;
            max-width: 480px;
            background-color: #1c1b22;
            border: 1px solid var(--border-subtle);
            border-radius: 6px;
            padding: 9px 12px;
            color: #fbfbfe;
            font-size: 14px;
            outline: none;
            transition: border-color 0.15s ease;
        }
        .select-box:focus, .input-box:focus {
            border-color: var(--border-focus);
            box-shadow: 0 0 0 1px var(--border-focus);
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">⚙️ Browser Settings</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip">🛡️ Shields</a>
            <a href="norva://permissions" class="chip">📷 Permissions</a>
            <a href="norva://cookies" class="chip">🍪 Storage & Cache</a>
            <a href="norva://settings" class="chip active">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <h2>Browser Settings & Engine Configuration</h2>
        <p style="color: var(--text-muted); margin-bottom: 24px;">Dedicated SQLite settings.db parameter catalog</p>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Primary Search Engine</h3>
            <div class="form-group">
                <label>Default Engine for Omnibox queries:</label>
                <select class="select-box" id="engine-select" onchange="changeDefaultEngine()">
                    {{ENGINE_OPTIONS}}
                </select>
                <p style="color: var(--text-muted); font-size: 12px; margin-top: 6px;">
                    Supports Google, Microsoft Bing, DuckDuckGo, Yahoo, and Ahmia (.onion Tor Search) out of the box.
                </p>
            </div>
        </div>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Tor Hidden Services & Onion Proxy</h3>
            <div class="form-group">
                <label>Tor SOCKS5 Proxy Host & Port:</label>
                <input type="text" class="input-box" value="127.0.0.1:9050" readonly />
                <p style="color: var(--text-muted); font-size: 12px; margin-top: 6px;">
                    Used when conducting .onion searches via Ahmia (!ahmia) or Tor hidden service proxies.
                </p>
            </div>
        </div>

        <div class="card">
            <h3 style="margin-bottom: 16px;">Add Custom Search Provider</h3>
            <div class="form-group">
                <label>Engine Name:</label>
                <input type="text" class="input-box" id="custom-name" placeholder="e.g. SearXNG" />
            </div>
            <div class="form-group">
                <label>URL Template (use {} for search query):</label>
                <input type="text" class="input-box" id="custom-template" placeholder="https://searx.example.com/search?q={}" />
            </div>
            <div class="form-group">
                <label>Bang Shortcut:</label>
                <input type="text" class="input-box" id="custom-bang" placeholder="e.g. !sx" />
            </div>
            <button class="btn" onclick="addCustomEngine()">Register New Engine</button>
        </div>
    </main>

    <script>
        function changeDefaultEngine() {
            const sel = document.getElementById("engine-select").value;
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "set_default_engine", id: sel }));
                alert("Default search engine updated to " + sel);
            }
        }

        function addCustomEngine() {
            const name = document.getElementById("custom-name").value.trim();
            const template = document.getElementById("custom-template").value.trim();
            const bang = document.getElementById("custom-bang").value.trim();

            if (!name || !template || !bang) {
                alert("Please complete all custom search engine fields.");
                return;
            }

            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({
                    action: "add_engine",
                    name: name,
                    template: template,
                    bang: bang
                }));
                alert("Custom engine added!");
                setTimeout(() => window.location.reload(), 150);
            }
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{ENGINE_OPTIONS}}", &engine_options)
}
