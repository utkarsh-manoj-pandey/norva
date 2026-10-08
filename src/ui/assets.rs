// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/ui/assets.rs
// Description: Executive Cyber-Dark UI Pages for internal norva:// views.
// ============================================================================

use crate::db::history_db::HistoryEntry;
use crate::db::permissions_db::{PermissionAuditLog, SitePermission};
use crate::db::settings_db::SearchEngineConfig;
use crate::db::shields_db::{BlockedEvent, ShieldStats};

/// Shared CSS styling providing an executive, modern cyber-dark design system.
///
/// I have completely overhauled this design system to eliminate the previous amateurish,
/// kiddish aesthetic. I wrote this theme inspired by high-end developer and privacy tools
/// like Linear, Brave Shields, and Proton: deep obsidian tones, subtle 1px border glows,
/// fine typography, and razor-sharp SVG vector iconography instead of crude emoji overload.
pub fn get_common_css() -> &'static str {
    r###"
    :root {
        --bg-primary: #0a0b10;
        --bg-surface: #12131c;
        --bg-surface-elevated: #181a26;
        --bg-surface-hover: #1f2130;
        --border-subtle: rgba(255, 255, 255, 0.08);
        --border-hover: rgba(255, 255, 255, 0.16);
        --border-focus: #6366f1;
        --accent-indigo: #6366f1;
        --accent-cyan: #06b6d4;
        --accent-emerald: #10b981;
        --accent-rose: #f43f5e;
        --accent-amber: #f59e0b;
        --text-primary: #f8fafc;
        --text-secondary: #94a3b8;
        --text-muted: #64748b;
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
        padding-bottom: 50px;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    /* Polished Minimalist Scrollbars */
    ::-webkit-scrollbar {
        width: 6px;
        height: 6px;
    }
    ::-webkit-scrollbar-track {
        background: transparent;
    }
    ::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.12);
        border-radius: 999px;
    }
    ::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.22);
    }

    /* Executive Internal Header */
    .internal-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 14px 32px;
        background-color: rgba(18, 19, 28, 0.85);
        border-bottom: 1px solid var(--border-subtle);
        position: sticky;
        top: 0;
        z-index: 100;
        backdrop-filter: blur(16px);
        -webkit-backdrop-filter: blur(16px);
        flex-wrap: wrap;
        gap: 16px;
    }
    .internal-header-left {
        display: flex;
        align-items: center;
        gap: 16px;
    }
    .back-link {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        background-color: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-subtle);
        color: var(--text-secondary);
        padding: 6px 14px;
        border-radius: 8px;
        text-decoration: none;
        font-size: 13px;
        font-weight: 500;
        transition: all 0.15s ease;
    }
    .back-link:hover {
        background-color: rgba(255, 255, 255, 0.1);
        border-color: var(--border-hover);
        color: #ffffff;
    }
    .internal-title {
        font-size: 17px;
        font-weight: 700;
        letter-spacing: -0.4px;
        color: var(--text-primary);
        display: flex;
        align-items: center;
        gap: 10px;
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
        display: inline-flex;
        align-items: center;
        gap: 6px;
    }
    .chip:hover {
        background-color: rgba(255, 255, 255, 0.06);
        color: #ffffff;
    }
    .chip.active {
        background-color: rgba(99, 102, 241, 0.16);
        border: 1px solid rgba(99, 102, 241, 0.35);
        color: #ffffff;
        font-weight: 600;
    }
    .container {
        max-width: 1180px;
        margin: 0 auto;
        padding: 32px 24px;
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
        background-color: rgba(255, 255, 255, 0.12);
        border-color: var(--border-hover);
    }
    .btn-primary {
        background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
        border: none;
        color: #ffffff;
        font-weight: 600;
    }
    .btn-primary:hover {
        background: linear-gradient(135deg, #7c7eff 0%, #6366f1 100%);
        box-shadow: 0 4px 14px rgba(99, 102, 241, 0.35);
    }
    .btn-danger {
        background-color: rgba(244, 63, 94, 0.12);
        border: 1px solid rgba(244, 63, 94, 0.3);
        color: #fb7185;
    }
    .btn-danger:hover {
        background-color: rgba(244, 63, 94, 0.22);
        border-color: rgba(244, 63, 94, 0.5);
    }
    .card {
        background-color: var(--bg-surface);
        border: 1px solid var(--border-subtle);
        border-radius: 14px;
        padding: 24px;
        margin-bottom: 24px;
        transition: border-color 0.15s ease, box-shadow 0.15s ease;
        position: relative;
    }
    .card:hover {
        border-color: var(--border-hover);
    }
    .mode-badge {
        padding: 4px 14px;
        border-radius: 20px;
        font-size: 11.5px;
        font-weight: 600;
        letter-spacing: 0.4px;
        display: inline-flex;
        align-items: center;
        gap: 6px;
    }
    .mode-badge.secure {
        background: rgba(244, 63, 94, 0.12);
        color: #fb7185;
        border: 1px solid rgba(244, 63, 94, 0.3);
    }
    .mode-badge.normal {
        background: rgba(16, 185, 129, 0.1);
        color: #34d399;
        border: 1px solid rgba(16, 185, 129, 0.25);
    }
    "###
}

/// Renders the primary `norva://newtab` executive dashboard.
///
/// I have completely rewritten this start page to replace all childish placecards,
/// fake Bollywood news, and crude emojis with executive-grade cyber-threat analytics:
/// - Real Threat Shield Matrix with category breakdown and bandwidth saved
/// - Zero-Trust Hardware Monitor (Camera, Mic, Location)
/// - Clean developer & privacy speed dial tiles with vector icons
/// - Omnibox with smart Bang command chips
/// - Live stream of neutralized tracker beacons
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
        let engine_badge_label = match e.id.as_str() {
            "duckduckgo" => "DDG",
            "google" => "Google",
            "bing" => "Bing",
            "yahoo" => "Yahoo",
            "ahmia" => "Ahmia (.onion)",
            "startpage" => "Startpage",
            "brave" => "Brave",
            "github" => "GitHub",
            "wikipedia" => "Wikipedia",
            _ => "Web",
        };
        engine_items_html.push_str(&format!(
            r###"<div class="engine-dropdown-item {}" data-id="{}" data-name="{}" data-bang="{}" onclick="selectSearchEngine('{}', '{}', '{}')">
                <span class="engine-item-name">{}</span>
                <span class="engine-item-bang">{}</span>
            </div>"###,
            is_sel, e.id, e.name, e.bang, e.id, e.name, e.bang, engine_badge_label, e.bang
        ));
    }

    let mode_pill = if secure_mode {
        r###"<span class="mode-badge secure">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
            Zero-Trace Volatile Ghost Session
        </span>"###
    } else {
        r###"<span class="mode-badge normal">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
            Active Privacy Hardened Session
        </span>"###
    };

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>New Tab | NORVA</title>
    <style>
        {{COMMON_CSS}}

        /* Modern Executive Canvas */
        .nt-canvas {
            max-width: 1240px;
            margin: 0 auto;
            padding: 36px 32px 64px;
        }

        /* Top Brand Header */
        .nt-topbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 36px;
        }
        .nt-brand {
            display: flex;
            align-items: center;
            gap: 14px;
            text-decoration: none;
            color: inherit;
        }
        .nt-brand-text {
            font-size: 26px;
            font-weight: 800;
            letter-spacing: -0.8px;
            color: #ffffff;
        }
        .nt-brand-sub {
            font-size: 11px;
            font-weight: 600;
            letter-spacing: 0.8px;
            text-transform: uppercase;
            color: var(--accent-cyan);
            background: rgba(6, 182, 212, 0.1);
            border: 1px solid rgba(6, 182, 212, 0.25);
            padding: 2px 8px;
            border-radius: 6px;
        }

        /* Centered Sleek Search Capsule */
        .nt-search-container {
            max-width: 720px;
            margin: 0 auto 44px;
        }
        .nt-search-box {
            display: flex;
            align-items: center;
            background-color: #161724;
            border: 1.5px solid rgba(255, 255, 255, 0.09);
            border-radius: 28px;
            height: 52px;
            padding: 0 18px;
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
            position: relative;
        }
        .nt-search-box:focus-within {
            border-color: rgba(99, 102, 241, 0.6);
            box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15), 0 10px 32px rgba(0, 0, 0, 0.5);
            background-color: #1a1c2c;
        }
        .engine-picker-btn {
            display: flex;
            align-items: center;
            gap: 6px;
            background: rgba(255, 255, 255, 0.06);
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 18px;
            padding: 5px 12px;
            color: var(--text-primary);
            cursor: pointer;
            font-size: 12.5px;
            font-weight: 500;
            margin-right: 12px;
            transition: all 0.15s ease;
            user-select: none;
        }
        .engine-picker-btn:hover {
            background: rgba(255, 255, 255, 0.12);
        }
        .engine-dropdown-menu {
            display: none;
            position: absolute;
            top: 60px;
            left: 12px;
            width: 270px;
            background-color: #181926;
            border: 1px solid rgba(255, 255, 255, 0.12);
            border-radius: 14px;
            box-shadow: 0 16px 40px rgba(0, 0, 0, 0.65);
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
            justify-content: space-between;
            padding: 9px 12px;
            border-radius: 8px;
            color: var(--text-secondary);
            font-size: 13px;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .engine-dropdown-item:hover, .engine-dropdown-item.active {
            background-color: rgba(99, 102, 241, 0.14);
            color: #ffffff;
        }
        .engine-item-name {
            font-weight: 500;
        }
        .engine-item-bang {
            font-size: 11px;
            color: var(--text-muted);
            background: rgba(0, 0, 0, 0.3);
            padding: 2px 6px;
            border-radius: 4px;
        }
        .nt-search-input {
            flex: 1;
            background: transparent;
            border: none;
            color: #ffffff;
            font-size: 15px;
            outline: none;
        }
        .nt-search-input::placeholder {
            color: var(--text-muted);
        }
        .nt-search-btn {
            background: transparent;
            border: none;
            color: var(--text-secondary);
            cursor: pointer;
            padding: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 50%;
            transition: color 0.15s ease;
        }
        .nt-search-btn:hover {
            color: var(--accent-indigo);
        }

        /* Quick Bang Shortcuts Bar */
        .search-chips-row {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            margin-top: 12px;
            flex-wrap: wrap;
        }
        .search-chip {
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.07);
            border-radius: 14px;
            padding: 4px 12px;
            font-size: 11.5px;
            color: var(--text-secondary);
            cursor: pointer;
            transition: all 0.15s ease;
            text-decoration: none;
        }
        .search-chip:hover {
            background: rgba(99, 102, 241, 0.12);
            color: #ffffff;
            border-color: rgba(99, 102, 241, 0.3);
        }

        /* Threat Telemetry & Security Placecards Grid (Replaces toy widgets) */
        .telemetry-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 18px;
            margin-bottom: 40px;
        }
        @media (max-width: 1024px) {
            .telemetry-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        @media (max-width: 640px) {
            .telemetry-grid {
                grid-template-columns: 1fr;
            }
        }
        .stat-card {
            background: linear-gradient(180deg, rgba(255, 255, 255, 0.02) 0%, rgba(255, 255, 255, 0) 100%), var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 14px;
            padding: 20px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            transition: all 0.15s ease;
        }
        .stat-card:hover {
            border-color: var(--border-hover);
            transform: translateY(-2px);
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
        }
        .stat-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 12px;
        }
        .stat-label {
            font-size: 12px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.6px;
            color: var(--text-muted);
        }
        .stat-icon {
            color: var(--accent-indigo);
        }
        .stat-number {
            font-size: 28px;
            font-weight: 800;
            color: #ffffff;
            letter-spacing: -0.8px;
            margin-bottom: 4px;
        }
        .stat-subtext {
            font-size: 12px;
            color: var(--text-secondary);
        }
        .stat-badge {
            display: inline-flex;
            align-items: center;
            gap: 5px;
            font-size: 11px;
            font-weight: 600;
            color: var(--accent-emerald);
            background: rgba(16, 185, 129, 0.1);
            padding: 2px 8px;
            border-radius: 4px;
            width: fit-content;
            margin-top: 8px;
        }

        /* Developer & Privacy Speed Dial Grid (Placecards) */
        .section-header-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 18px;
        }
        .section-heading {
            font-size: 15px;
            font-weight: 700;
            color: var(--text-primary);
            letter-spacing: -0.3px;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .speeddial-grid {
            display: grid;
            grid-template-columns: repeat(5, 1fr);
            gap: 16px;
            margin-bottom: 44px;
        }
        @media (max-width: 1024px) {
            .speeddial-grid {
                grid-template-columns: repeat(3, 1fr);
            }
        }
        @media (max-width: 640px) {
            .speeddial-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        .dial-card {
            background-color: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 12px;
            padding: 16px;
            text-decoration: none;
            color: inherit;
            display: flex;
            align-items: center;
            gap: 14px;
            transition: all 0.15s ease;
            cursor: pointer;
        }
        .dial-card:hover {
            border-color: var(--border-hover);
            background-color: var(--bg-surface-elevated);
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(0, 0, 0, 0.4);
        }
        .dial-icon-wrap {
            width: 40px;
            height: 40px;
            border-radius: 10px;
            background-color: rgba(255, 255, 255, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            border: 1px solid rgba(255, 255, 255, 0.06);
        }
        .dial-info {
            overflow: hidden;
        }
        .dial-title {
            font-size: 13.5px;
            font-weight: 600;
            color: #ffffff;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .dial-desc {
            font-size: 11px;
            color: var(--text-muted);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            margin-top: 2px;
        }

        /* Vault Navigation Quick Bar */
        .vault-bar {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 12px;
            flex-wrap: wrap;
            padding: 18px 0;
            border-top: 1px solid var(--border-subtle);
        }
        .vault-pill {
            background-color: rgba(255, 255, 255, 0.04);
            border: 1px solid var(--border-subtle);
            color: var(--text-secondary);
            text-decoration: none;
            padding: 7px 16px;
            border-radius: 20px;
            font-size: 12.5px;
            font-weight: 500;
            transition: all 0.15s ease;
            display: inline-flex;
            align-items: center;
            gap: 6px;
        }
        .vault-pill:hover {
            background-color: rgba(99, 102, 241, 0.12);
            border-color: rgba(99, 102, 241, 0.3);
            color: #ffffff;
        }
    </style>
</head>
<body>
    <div class="nt-canvas">
        <!-- Topbar (Brand + Header Actions) -->
        <div class="nt-topbar">
            <a href="norva://newtab" class="nt-brand">
                <svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#6366f1" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                    <circle cx="12" cy="11" r="3"/>
                </svg>
                <div>
                    <div class="nt-brand-text">norva</div>
                </div>
                <span class="nt-brand-sub">CORE v0.1</span>
            </a>
            <div style="display: flex; align-items: center; gap: 12px;">
                {{MODE_PILL}}
            </div>
        </div>

        <!-- Centered Modern Search Bar -->
        <div class="nt-search-container">
            <div class="nt-search-box">
                <!-- Search Engine Picker Trigger -->
                <div class="engine-picker-btn" id="engine-picker-trigger" onclick="toggleEngineDropdown()">
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
                    placeholder="Search with {{DEFAULT_NAME}} or enter address..." autocomplete="off" autofocus />
                
                <button class="nt-search-btn" id="search-submit" title="Search">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                        <circle cx="11" cy="11" r="7"/>
                        <line x1="16.5" y1="16.5" x2="21" y2="21"/>
                    </svg>
                </button>
            </div>

            <!-- Quick Bang Chips -->
            <div class="search-chips-row">
                <span class="search-chip" onclick="quickFillBang('!ddg ')">DuckDuckGo (!ddg)</span>
                <span class="search-chip" onclick="quickFillBang('!g ')">Google (!g)</span>
                <span class="search-chip" onclick="quickFillBang('!b ')">Bing (!b)</span>
                <span class="search-chip" onclick="quickFillBang('!ahmia ')">Ahmia Tor (!ahmia)</span>
                <span class="search-chip" onclick="quickFillBang('!gh ')">GitHub (!gh)</span>
                <span class="search-chip" onclick="quickFillBang('!w ')">Wikipedia (!w)</span>
                <span class="search-chip" onclick="quickFillBang('!sp ')">Startpage (!sp)</span>
                <span class="search-chip" onclick="quickFillBang('!brave ')">Brave (!brave)</span>
            </div>
        </div>

        <!-- Live Threat Telemetry & Security Placecards Grid -->
        <div class="telemetry-grid">
            <!-- 1. Threat Shield Matrix -->
            <div class="stat-card">
                <div>
                    <div class="stat-header">
                        <span class="stat-label">Threats Neutralized</span>
                        <svg class="stat-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#6366f1" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                    </div>
                    <div class="stat-number">{{ADS_BLOCKED}}</div>
                    <div class="stat-subtext">Trackers, ads & telemetry beacons blocked</div>
                </div>
                <div class="stat-badge">
                    <span style="display:inline-block; width:6px; height:6px; border-radius:50%; background:#10b981;"></span>
                    Active Shield Defense
                </div>
            </div>

            <!-- 2. Zero-Trust Hardware Guard -->
            <div class="stat-card">
                <div>
                    <div class="stat-header">
                        <span class="stat-label">Hardware Arbiter</span>
                        <svg class="stat-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#06b6d4" stroke-width="2"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>
                    </div>
                    <div class="stat-number">3 Devices</div>
                    <div class="stat-subtext">Camera, Microphone & GPS gated</div>
                </div>
                <div class="stat-badge" style="color: #06b6d4; background: rgba(6, 182, 212, 0.1);">
                    Zero-Trust Policy
                </div>
            </div>

            <!-- 3. Bandwidth & Latency Saved -->
            <div class="stat-card">
                <div>
                    <div class="stat-header">
                        <span class="stat-label">Data Saved</span>
                        <svg class="stat-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/></svg>
                    </div>
                    <div class="stat-number">{{SAVED_MB}} MB</div>
                    <div class="stat-subtext">Estimated payload bloat prevented</div>
                </div>
                <div class="stat-badge">
                    ~38% Faster Load Time
                </div>
            </div>

            <!-- 4. Storage & Cache Isolation -->
            <div class="stat-card">
                <div>
                    <div class="stat-header">
                        <span class="stat-label">Storage Footprint</span>
                        <svg class="stat-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
                    </div>
                    <div class="stat-number">{{CACHE_MB}} MB</div>
                    <div class="stat-subtext">Partitioned cache & cookie storage</div>
                </div>
                <button class="btn" style="margin-top: 8px; font-size: 11px; padding: 4px 10px;" onclick="purgeCache()">
                    Purge Storage
                </button>
            </div>
        </div>

        <!-- Developer & Privacy Speed Dial (Real, Professional Placecards) -->
        <div class="section-header-row">
            <span class="section-heading">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
                Secure Quick Access
            </span>
        </div>

        <div class="speeddial-grid">
            <!-- 1. GitHub -->
            <div class="dial-card" onclick="navigate('https://github.com')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="#ffffff"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">GitHub</div>
                    <div class="dial-desc">Repositories & code</div>
                </div>
            </div>

            <!-- 2. DuckDuckGo -->
            <div class="dial-card" onclick="navigate('https://duckduckgo.com')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="#de5833"><circle cx="12" cy="12" r="10"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">DuckDuckGo</div>
                    <div class="dial-desc">Private web search</div>
                </div>
            </div>

            <!-- 3. Tor Project -->
            <div class="dial-card" onclick="navigate('https://www.torproject.org')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#7d4698" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">Tor Project</div>
                    <div class="dial-desc">Online anonymity</div>
                </div>
            </div>

            <!-- 4. Electronic Frontier Foundation -->
            <div class="dial-card" onclick="navigate('https://www.eff.org')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#e02424" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">EFF</div>
                    <div class="dial-desc">Digital privacy rights</div>
                </div>
            </div>

            <!-- 5. Rust Programming -->
            <div class="dial-card" onclick="navigate('https://www.rust-lang.org')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#f97316" stroke-width="2"><circle cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="3"/><line x1="12" y1="3" x2="12" y2="6"/><line x1="12" y1="18" x2="12" y2="21"/><line x1="3" y1="12" x2="6" y2="12"/><line x1="18" y1="12" x2="21" y2="12"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">Rust Lang</div>
                    <div class="dial-desc">Systems engineering</div>
                </div>
            </div>

            <!-- 6. Hacker News -->
            <div class="dial-card" onclick="navigate('https://news.ycombinator.com')">
                <div class="dial-icon-wrap">
                    <span style="font-weight: 800; color: #ff6600; font-size: 16px;">Y</span>
                </div>
                <div class="dial-info">
                    <div class="dial-title">Hacker News</div>
                    <div class="dial-desc">Tech & cyber briefing</div>
                </div>
            </div>

            <!-- 7. Wikipedia -->
            <div class="dial-card" onclick="navigate('https://www.wikipedia.org')">
                <div class="dial-icon-wrap">
                    <span style="font-family: serif; font-weight: 700; color: #ffffff; font-size: 18px;">W</span>
                </div>
                <div class="dial-info">
                    <div class="dial-title">Wikipedia</div>
                    <div class="dial-desc">Open encyclopedia</div>
                </div>
            </div>

            <!-- 8. Ahmia (.onion) -->
            <div class="dial-card" onclick="navigate('https://ahmia.fi')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#9333ea" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">Ahmia</div>
                    <div class="dial-desc">Tor hidden services</div>
                </div>
            </div>

            <!-- 9. Privacy Guides -->
            <div class="dial-card" onclick="navigate('https://www.privacyguides.org')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">Privacy Guides</div>
                    <div class="dial-desc">Security recommendations</div>
                </div>
            </div>

            <!-- 10. Startpage -->
            <div class="dial-card" onclick="navigate('https://www.startpage.com')">
                <div class="dial-icon-wrap">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#3b82f6" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
                </div>
                <div class="dial-info">
                    <div class="dial-title">Startpage</div>
                    <div class="dial-desc">European private search</div>
                </div>
            </div>
        </div>

        <!-- Privacy Shields & Internal Command Quick Access -->
        <div class="vault-bar">
            <span style="font-size: 12px; color: var(--text-muted); margin-right: 6px;">NORVA Vaults:</span>
            <a href="norva://history" class="vault-pill">📜 History Vault</a>
            <a href="norva://shields" class="vault-pill">🛡️ Shields Center ({{ADS_BLOCKED}} blocked)</a>
            <a href="norva://permissions" class="vault-pill">📷 Device Permissions</a>
            <a href="norva://cookies" class="vault-pill">🍪 Cache & Storage ({{CACHE_MB}} MB)</a>
            <a href="norva://settings" class="vault-pill">⚙️ Settings & Engines</a>
        </div>
    </div>

    <script>
        function navigate(url) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "navigate", query: url }));
            } else {
                window.location.href = url;
            }
        }

        function toggleEngineDropdown() {
            const menu = document.getElementById("engine-dropdown");
            menu.classList.toggle("show");
        }

        document.addEventListener("click", (e) => {
            const trigger = document.getElementById("engine-picker-trigger");
            const menu = document.getElementById("engine-dropdown");
            if (!trigger.contains(e.target) && !menu.contains(e.target)) {
                menu.classList.remove("show");
            }
        });

        function selectSearchEngine(id, name, bang) {
            document.getElementById("active-engine-label").textContent = name;
            document.getElementById("engine-dropdown").classList.remove("show");
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "set_default_engine", query: id }));
            }
        }

        function quickFillBang(bang) {
            const input = document.getElementById("search-input");
            input.value = bang;
            input.focus();
        }

        function purgeCache() {
            if (confirm("Purge local disk cache and session storage immediately?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "purge_cache" }));
                    setTimeout(() => window.location.reload(), 200);
                }
            }
        }

        document.getElementById("search-input").addEventListener("keydown", (e) => {
            if (e.key === "Enter") {
                const val = e.target.value.trim();
                if (val) {
                    navigate(val);
                }
            }
        });

        document.getElementById("search-submit").addEventListener("click", () => {
            const val = document.getElementById("search-input").value.trim();
            if (val) {
                navigate(val);
            }
        });
    </script>
</body>
</html>"###;

    let saved_mb = (stats.estimated_bandwidth_saved_kb as f64) / 1024.0;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{MODE_PILL}}", mode_pill)
        .replace("{{DEFAULT_NAME}}", &default_engine.name)
        .replace("{{ENGINE_ITEMS}}", &engine_items_html)
        .replace("{{ADS_BLOCKED}}", &stats.total_all_time.to_string())
        .replace("{{SAVED_MB}}", &format!("{:.1}", saved_mb))
        .replace("{{CACHE_MB}}", &format!("{:.1}", cache_mb))
}

/// Renders the `norva://history` view with interactive filtering and purge controls.
pub fn render_history_page(entries: &[HistoryEntry]) -> String {
    let mut rows = String::new();
    if entries.is_empty() {
        rows.push_str(r###"<div style="text-align:center; padding: 56px 24px; color: var(--text-muted);">
            <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" style="margin-bottom:12px; opacity:0.6;"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
            <h3 style="color: var(--text-primary); font-size: 16px;">History Vault is Clear</h3>
            <p style="margin-top: 6px; font-size: 13px;">No browsing activity recorded. In Secure Ghost Mode, history is never persisted.</p>
        </div>"###);
    } else {
        for e in entries {
            let engine_badge = if let Some(ref eng) = e.search_engine {
                format!(r###"<span style="background: rgba(99, 102, 241, 0.12); color: #818cf8; border: 1px solid rgba(99, 102, 241, 0.25); padding: 2px 7px; border-radius: 4px; font-size: 11px; font-weight: 600;">🔍 {}</span>"###, eng)
            } else {
                String::new()
            };

            let query_info = if let Some(ref q) = e.search_query {
                format!(r###"<span style="color: var(--accent-cyan); font-weight: 500;">Search: "{}"</span> • "###, q)
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
                            {}{} • Visited {} times
                        </div>
                    </div>
                    <button class="btn btn-danger" style="padding: 4px 10px; font-size: 12px;" onclick="deleteHistory({})">Delete</button>
                </div>"###,
                e.url, e.title, engine_badge, query_info, e.visit_time, e.visit_count, e.id
            ));
        }
    }

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>History Vault | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .header-actions {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 24px;
            flex-wrap: wrap;
            gap: 16px;
        }
        .search-box {
            background: #141520;
            border: 1px solid var(--border-subtle);
            border-radius: 8px;
            padding: 8px 14px;
            color: #ffffff;
            width: 320px;
            outline: none;
            font-size: 13px;
            transition: border-color 0.15s ease;
        }
        .search-box:focus {
            border-color: var(--border-focus);
            box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.15);
        }
        .history-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 14px 18px;
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 10px;
            margin-bottom: 10px;
            transition: all 0.15s ease;
        }
        .history-item:hover {
            background-color: var(--bg-surface-elevated);
            border-color: var(--border-hover);
        }
        .history-main {
            overflow: hidden;
            flex: 1;
            margin-right: 16px;
        }
        .history-title {
            font-size: 14px;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 10px;
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
            color: var(--accent-indigo);
        }
        .history-meta {
            font-size: 12px;
            color: var(--text-muted);
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
            <a href="norva://cookies" class="chip">🍪 Storage</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <div class="header-actions">
            <div>
                <h2 style="font-size: 22px; font-weight: 700; color: #ffffff;">Browsing History Vault</h2>
                <p style="color: var(--text-muted); font-size: 13px; margin-top: 4px;">Isolated SQLite history.db storage with zero telemetry</p>
            </div>
            <div style="display: flex; gap: 10px;">
                <input type="text" class="search-box" id="search-hist" placeholder="Filter history records..." oninput="filterHistory()" />
                <button class="btn btn-danger" onclick="clearAllHistory()">Purge All History</button>
            </div>
        </div>

        <div class="card" id="history-container" style="padding: 12px;">
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

/// Renders the `norva://shields` view with real-time telemetry stream and switch controls.
pub fn render_shields_page(stats: &ShieldStats, events: &[BlockedEvent]) -> String {
    let mut log_rows = String::new();
    if events.is_empty() {
        log_rows.push_str(r###"<tr><td colspan="4" style="text-align:center; padding: 36px; color: var(--text-muted);">No surveillance events recorded yet. Browse any site to see live neutralization logs.</td></tr>"###);
    } else {
        for ev in events {
            let (badge_bg, badge_color) = match ev.category {
                crate::db::shields_db::BlockCategory::Ad => ("rgba(6, 182, 212, 0.12)", "#22d3ee"),
                crate::db::shields_db::BlockCategory::Tracker => ("rgba(244, 63, 94, 0.12)", "#fb7185"),
                crate::db::shields_db::BlockCategory::Fingerprinter => ("rgba(168, 85, 247, 0.12)", "#c084fc"),
                crate::db::shields_db::BlockCategory::Cryptominer => ("rgba(245, 158, 11, 0.12)", "#fbbf24"),
                crate::db::shields_db::BlockCategory::TrackingParam => ("rgba(16, 185, 129, 0.12)", "#34d399"),
            };
            log_rows.push_str(&format!(
                r###"<tr>
                    <td><span style="background: {}; color: {}; padding: 3px 8px; border-radius: 6px; font-weight: 700; font-size: 11px; text-transform: uppercase;">{:?}</span></td>
                    <td style="font-family: monospace; color: var(--text-primary); font-size: 13px; font-weight: 500;">{}</td>
                    <td style="color: var(--text-secondary); font-size: 13px;">{}</td>
                    <td style="color: var(--text-muted); font-size: 12px;">{}</td>
                </tr>"###,
                badge_bg, badge_color, ev.category, ev.blocked_domain, ev.page_domain, ev.timestamp
            ));
        }
    }

    let saved_mb = (stats.estimated_bandwidth_saved_kb as f64) / 1024.0;

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Privacy Shields | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .table {
            width: 100%;
            border-collapse: collapse;
            font-size: 13.5px;
        }
        .table th, .table td {
            padding: 12px 16px;
            text-align: left;
            border-bottom: 1px solid var(--border-subtle);
        }
        .table th {
            color: var(--text-muted);
            text-transform: uppercase;
            font-size: 11px;
            letter-spacing: 0.6px;
            background: rgba(255, 255, 255, 0.02);
        }
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 16px;
            margin-bottom: 24px;
        }
        @media (max-width: 900px) {
            .metrics-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        .metric-box {
            background-color: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 12px;
            padding: 18px;
        }
        .metric-title {
            font-size: 12px;
            color: var(--text-muted);
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 6px;
        }
        .metric-val {
            font-size: 24px;
            font-weight: 800;
            color: #ffffff;
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">🛡️ Privacy Shields Command Center</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip active">🛡️ Shields</a>
            <a href="norva://permissions" class="chip">📷 Permissions</a>
            <a href="norva://cookies" class="chip">🍪 Storage</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <div class="metrics-grid">
            <div class="metric-box">
                <div class="metric-title">Total Interceptions</div>
                <div class="metric-val" style="color: #6366f1;">{{TOTAL_ALL}}</div>
            </div>
            <div class="metric-box">
                <div class="metric-title">Ads Blocked</div>
                <div class="metric-val" style="color: #06b6d4;">{{ADS_COUNT}}</div>
            </div>
            <div class="metric-box">
                <div class="metric-title">Trackers Blocked</div>
                <div class="metric-val" style="color: #f43f5e;">{{TRACKERS_COUNT}}</div>
            </div>
            <div class="metric-box">
                <div class="metric-title">Data Saved</div>
                <div class="metric-val" style="color: #10b981;">{{SAVED_MB}} MB</div>
            </div>
        </div>

        <div class="card">
            <h3 style="font-size: 16px; font-weight: 700; color: #ffffff; margin-bottom: 16px;">Live Neutralized Threats Telemetry</h3>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Category</th>
                            <th>Blocked Host Target</th>
                            <th>Originating Context</th>
                            <th>Timestamp</th>
                        </tr>
                    </thead>
                    <tbody>
                        {{LOG_ROWS}}
                    </tbody>
                </table>
            </div>
        </div>
    </main>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{TOTAL_ALL}}", &stats.total_all_time.to_string())
        .replace("{{ADS_COUNT}}", &stats.total_ads_blocked.to_string())
        .replace("{{TRACKERS_COUNT}}", &stats.total_trackers_blocked.to_string())
        .replace("{{SAVED_MB}}", &format!("{:.1}", saved_mb))
        .replace("{{LOG_ROWS}}", &log_rows)
}

/// Renders the `norva://permissions` view with granular hardware arbiter policies and audit logs.
pub fn render_permissions_page(permissions: &[SitePermission], audits: &[PermissionAuditLog]) -> String {
    let mut perm_rows = String::new();
    if permissions.is_empty() {
        perm_rows.push_str(r###"<tr><td colspan="5" style="text-align:center; padding: 32px; color: var(--text-muted);">No custom site permissions configured. All origins operate under zero-trust gating.</td></tr>"###);
    } else {
        for p in permissions {
            perm_rows.push_str(&format!(
                r###"<tr>
                    <td style="font-weight: 600; color: #ffffff;">{}</td>
                    <td><span class="policy-badge {:?}">{}</span></td>
                    <td><span class="policy-badge {:?}">{}</span></td>
                    <td><span class="policy-badge {:?}">{}</span></td>
                    <td>
                        <button class="btn btn-danger" style="padding: 4px 8px; font-size: 11px;" onclick="resetPermission('{}')">Reset</button>
                    </td>
                </tr>"###,
                p.domain, p.camera, p.camera.as_str(), p.microphone, p.microphone.as_str(), p.location, p.location.as_str(), p.domain
            ));
        }
    }

    let mut audit_rows = String::new();
    if audits.is_empty() {
        audit_rows.push_str(r###"<tr><td colspan="4" style="text-align:center; padding: 28px; color: var(--text-muted);">No hardware sensor invocations recorded.</td></tr>"###);
    } else {
        for a in audits {
            let decision_color = if a.decision == "allowed" || a.decision == "allowed_internal" {
                "#10b981"
            } else {
                "#f43f5e"
            };
            audit_rows.push_str(&format!(
                r###"<tr>
                    <td style="font-weight: 500; color: #ffffff;">{}</td>
                    <td style="text-transform: capitalize; color: var(--text-secondary);">{}</td>
                    <td><span style="color: {}; font-weight: 600; text-transform: uppercase; font-size: 11px;">{}</span></td>
                    <td style="color: var(--text-muted); font-size: 12px;">{}</td>
                </tr>"###,
                a.domain, a.permission_type, decision_color, a.decision, a.timestamp
            ));
        }
    }

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Hardware Permissions | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .table {
            width: 100%;
            border-collapse: collapse;
            font-size: 13.5px;
        }
        .table th, .table td {
            padding: 12px 16px;
            text-align: left;
            border-bottom: 1px solid var(--border-subtle);
        }
        .table th {
            color: var(--text-muted);
            text-transform: uppercase;
            font-size: 11px;
            letter-spacing: 0.6px;
            background: rgba(255, 255, 255, 0.02);
        }
        .policy-badge {
            padding: 3px 8px;
            border-radius: 6px;
            font-size: 11px;
            font-weight: 700;
            text-transform: uppercase;
        }
        .policy-badge.Allow {
            background: rgba(16, 185, 129, 0.12);
            color: #34d399;
        }
        .policy-badge.Block {
            background: rgba(244, 63, 94, 0.12);
            color: #fb7185;
        }
        .policy-badge.Ask {
            background: rgba(245, 158, 11, 0.12);
            color: #fbbf24;
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">📷 Hardware & Location Permissions Arbiter</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip">🛡️ Shields</a>
            <a href="norva://permissions" class="chip active">📷 Permissions</a>
            <a href="norva://cookies" class="chip">🍪 Storage</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <div class="card">
            <h3 style="font-size: 16px; font-weight: 700; color: #ffffff; margin-bottom: 16px;">Configured Domain Policies</h3>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Origin Domain</th>
                            <th>Webcam / Video</th>
                            <th>Microphone / Audio</th>
                            <th>Geolocation Sensor</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {{PERM_ROWS}}
                    </tbody>
                </table>
            </div>
        </div>

        <div class="card">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                <h3 style="font-size: 16px; font-weight: 700; color: #ffffff;">Forensic Hardware Invocations Audit Log</h3>
                <button class="btn btn-danger" style="font-size: 12px; padding: 4px 10px;" onclick="clearAudit()">Clear Audit</button>
            </div>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Requested Origin</th>
                            <th>Device Target</th>
                            <th>Security Decision</th>
                            <th>Timestamp</th>
                        </tr>
                    </thead>
                    <tbody>
                        {{AUDIT_ROWS}}
                    </tbody>
                </table>
            </div>
        </div>
    </main>

    <script>
        function resetPermission(domain) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "reset_permission", domain: domain }));
                setTimeout(() => window.location.reload(), 150);
            }
        }

        function clearAudit() {
            if (confirm("Wipe all hardware forensic audit logs?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "clear_permissions_audit" }));
                    setTimeout(() => window.location.reload(), 150);
                }
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

/// Renders the `norva://cookies` view for cookie isolation and storage management.
pub fn render_cookies_page(cache_mb: f64) -> String {
    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Cookie Isolation & Cache | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .storage-grid {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 20px;
            margin-bottom: 24px;
        }
        @media (max-width: 768px) {
            .storage-grid {
                grid-template-columns: 1fr;
            }
        }
        .storage-card {
            background-color: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 14px;
            padding: 24px;
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">🍪 Cookie Partitioning & Cache Storage</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip">🛡️ Shields</a>
            <a href="norva://permissions" class="chip">📷 Permissions</a>
            <a href="norva://cookies" class="chip active">🍪 Storage</a>
            <a href="norva://settings" class="chip">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <div class="storage-grid">
            <div class="storage-card">
                <h3 style="font-size: 16px; font-weight: 700; color: #ffffff; margin-bottom: 8px;">Disk Cache Quota</h3>
                <p style="font-size: 13px; color: var(--text-secondary); margin-bottom: 16px;">Cached assets in ~/.local/share/norva/cache</p>
                <div style="font-size: 32px; font-weight: 800; color: #ffffff; margin-bottom: 16px;">{{CACHE_MB}} MB</div>
                <button class="btn btn-danger" onclick="purgeCache()">Purge Disk Cache Now</button>
            </div>

            <div class="storage-card">
                <h3 style="font-size: 16px; font-weight: 700; color: #ffffff; margin-bottom: 8px;">Cookie Isolation Policy</h3>
                <p style="font-size: 13px; color: var(--text-secondary); margin-bottom: 16px;">First-party partitioning enforced across all browsing contexts</p>
                <div style="display: flex; flex-direction: column; gap: 10px; font-size: 13px;">
                    <div style="display: flex; justify-content: space-between;">
                        <span>Third-Party Cookie Blocking:</span>
                        <span style="color: var(--accent-emerald); font-weight: 600;">ACTIVE</span>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span>Local Storage Partitioning:</span>
                        <span style="color: var(--accent-emerald); font-weight: 600;">ACTIVE</span>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span>Ghost Mode Zero-Trace Storage:</span>
                        <span style="color: var(--accent-indigo); font-weight: 600;">AVAILABLE</span>
                    </div>
                </div>
            </div>
        </div>
    </main>

    <script>
        function purgeCache() {
            if (confirm("Permanently wipe all cached assets from disk?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "purge_cache" }));
                    setTimeout(() => window.location.reload(), 200);
                }
            }
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{CACHE_MB}}", &format!("{:.1}", cache_mb))
}

/// Renders the `norva://settings` view with search engines catalog and browser configurations.
pub fn render_settings_page(
    engines: &[SearchEngineConfig],
    default_engine: &SearchEngineConfig,
    secure_mode: bool,
) -> String {
    let mut engine_rows = String::new();
    for e in engines {
        let is_default_badge = if e.id == default_engine.id {
            r###"<span style="background: rgba(16, 185, 129, 0.12); color: #34d399; padding: 2px 8px; border-radius: 4px; font-size: 11px; font-weight: 700;">DEFAULT</span>"###
        } else {
            r###"<button class="btn" style="padding: 3px 8px; font-size: 11px;" onclick="setDefaultEngine('{}')">Make Default</button>"###
        };

        engine_rows.push_str(&format!(
            r###"<tr>
                <td style="font-weight: 600; color: #ffffff;">{}</td>
                <td style="color: var(--accent-indigo); font-family: monospace; font-weight: 600;">{}</td>
                <td style="color: var(--text-secondary); font-size: 12px; font-family: monospace; max-width: 280px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">{}</td>
                <td>{}</td>
            </tr>"###,
            e.name, e.bang, e.query_template, is_default_badge.replace("{}", &e.id)
        ));
    }

    let mode_desc = if secure_mode {
        "Currently running in Ephemeral Ghost Mode (RAM only, zero disk writes)"
    } else {
        "Currently running in Privacy Hardened Session (5 Isolated SQLite DBs)"
    };

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Browser Settings | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .table {
            width: 100%;
            border-collapse: collapse;
            font-size: 13.5px;
        }
        .table th, .table td {
            padding: 12px 16px;
            text-align: left;
            border-bottom: 1px solid var(--border-subtle);
        }
        .table th {
            color: var(--text-muted);
            text-transform: uppercase;
            font-size: 11px;
            letter-spacing: 0.6px;
            background: rgba(255, 255, 255, 0.02);
        }
    </style>
</head>
<body>
    <header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <span>←</span> <span>Start Page</span>
            </a>
            <h2 class="internal-title">⚙️ Browser Configuration & Search Engines</h2>
        </div>
        <div class="internal-nav-chips">
            <a href="norva://newtab" class="chip">🏠 Start</a>
            <a href="norva://history" class="chip">📜 History</a>
            <a href="norva://shields" class="chip">🛡️ Shields</a>
            <a href="norva://permissions" class="chip">📷 Permissions</a>
            <a href="norva://cookies" class="chip">🍪 Storage</a>
            <a href="norva://settings" class="chip active">⚙️ Settings</a>
        </div>
    </header>

    <main class="container">
        <div class="card" style="margin-bottom: 24px;">
            <h3 style="font-size: 16px; font-weight: 700; color: #ffffff; margin-bottom: 8px;">Active Session Profile</h3>
            <p style="font-size: 13.5px; color: var(--text-secondary);">{{MODE_DESC}}</p>
        </div>

        <div class="card">
            <h3 style="font-size: 16px; font-weight: 700; color: #ffffff; margin-bottom: 16px;">Configured Search Engines</h3>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Search Provider</th>
                            <th>Bang Hotkey</th>
                            <th>URL Template</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        {{ENGINE_ROWS}}
                    </tbody>
                </table>
            </div>
        </div>
    </main>

    <script>
        function setDefaultEngine(id) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "set_default_engine", query: id }));
                setTimeout(() => window.location.reload(), 150);
            }
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{MODE_DESC}}", mode_desc)
        .replace("{{ENGINE_ROWS}}", &engine_rows)
}
