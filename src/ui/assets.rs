// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/ui/assets.rs
// Description: Executive Minimalist Dark UI System for internal norva:// views.
//              Designed with authentic, Linear/Arc/Safari-inspired aesthetics.
// ============================================================================

use crate::db::bookmarks_db::BookmarkEntry;
use crate::db::downloads_db::DownloadEntry;
use crate::db::history_db::HistoryEntry;
use crate::db::passwords_db::PasswordEntry;
use crate::db::permissions_db::{PermissionAuditLog, SitePermission};
use crate::db::settings_db::SearchEngineConfig;
use crate::db::shields_db::{BlockedEvent, ShieldStats};

/// Shared CSS providing an executive, minimalist dark design system.
pub fn get_common_css() -> &'static str {
    r###"
    :root {
        --bg-primary: #0b0c10;
        --bg-surface: #12131b;
        --bg-surface-elevated: #181924;
        --bg-surface-hover: #1f202e;
        --border-subtle: rgba(255, 255, 255, 0.07);
        --border-hover: rgba(255, 255, 255, 0.14);
        --border-focus: #6366f1;
        --accent: #6366f1;
        --accent-emerald: #10b981;
        --accent-rose: #f43f5e;
        --accent-amber: #f59e0b;
        --text-primary: #f4f4f5;
        --text-secondary: #a1a1aa;
        --text-muted: #71717a;
    }
    * {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }
    body {
        font-family: -apple-system, BlinkMacSystemFont, 'Inter', 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
        background-color: var(--bg-primary);
        color: var(--text-primary);
        min-height: 100vh;
        overflow-x: hidden;
        padding-bottom: 40px;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    ::-webkit-scrollbar {
        width: 6px;
        height: 6px;
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

    .internal-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 28px;
        background-color: rgba(18, 19, 27, 0.88);
        border-bottom: 1px solid var(--border-subtle);
        position: sticky;
        top: 0;
        z-index: 100;
        backdrop-filter: blur(16px);
        -webkit-backdrop-filter: blur(16px);
        flex-wrap: wrap;
        gap: 14px;
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
        background-color: rgba(255, 255, 255, 0.04);
        border: 1px solid var(--border-subtle);
        color: var(--text-secondary);
        padding: 5px 12px;
        border-radius: 6px;
        text-decoration: none;
        font-size: 12.5px;
        font-weight: 500;
        transition: all 0.12s ease;
    }
    .back-link:hover {
        background-color: rgba(255, 255, 255, 0.08);
        border-color: var(--border-hover);
        color: #ffffff;
    }
    .internal-title {
        font-size: 15.5px;
        font-weight: 600;
        letter-spacing: -0.3px;
        color: var(--text-primary);
    }
    .internal-nav-chips {
        display: flex;
        align-items: center;
        gap: 4px;
        flex-wrap: wrap;
    }
    .chip {
        color: var(--text-secondary);
        text-decoration: none;
        padding: 5px 11px;
        border-radius: 6px;
        font-size: 12.5px;
        font-weight: 450;
        transition: all 0.12s ease;
        background: transparent;
    }
    .chip:hover {
        background-color: rgba(255, 255, 255, 0.05);
        color: #ffffff;
    }
    .chip.active {
        background-color: rgba(255, 255, 255, 0.08);
        border: 1px solid var(--border-hover);
        color: #ffffff;
        font-weight: 550;
    }

    .container {
        max-width: 1040px;
        margin: 0 auto;
        padding: 28px 20px;
    }
    .btn {
        background-color: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-subtle);
        color: var(--text-primary);
        border-radius: 6px;
        padding: 6px 14px;
        font-size: 12.5px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.12s ease;
        display: inline-flex;
        align-items: center;
        gap: 6px;
        text-decoration: none;
    }
    .btn:hover {
        background-color: rgba(255, 255, 255, 0.1);
        border-color: var(--border-hover);
    }
    .btn-primary {
        background: #6366f1;
        border: 1px solid #6366f1;
        color: #ffffff;
        font-weight: 550;
    }
    .btn-primary:hover {
        background: #4f46e5;
        border-color: #4f46e5;
    }
    .btn-danger {
        background-color: rgba(244, 63, 94, 0.08);
        border: 1px solid rgba(244, 63, 94, 0.25);
        color: #fb7185;
    }
    .btn-danger:hover {
        background-color: rgba(244, 63, 94, 0.18);
        border-color: rgba(244, 63, 94, 0.4);
    }

    .card {
        background-color: var(--bg-surface);
        border: 1px solid var(--border-subtle);
        border-radius: 12px;
        padding: 20px;
        margin-bottom: 20px;
        transition: border-color 0.12s ease;
    }
    .card:hover {
        border-color: var(--border-hover);
    }

    .table {
        width: 100%;
        border-collapse: collapse;
        font-size: 13px;
    }
    .table th, .table td {
        padding: 10px 14px;
        text-align: left;
        border-bottom: 1px solid var(--border-subtle);
    }
    .table th {
        color: var(--text-muted);
        text-transform: uppercase;
        font-size: 10.5px;
        font-weight: 600;
        letter-spacing: 0.5px;
        background: rgba(255, 255, 255, 0.015);
    }

    .form-input {
        background: #090a0f;
        border: 1px solid var(--border-subtle);
        border-radius: 6px;
        color: #ffffff;
        padding: 7px 12px;
        font-size: 13px;
        font-family: inherit;
        outline: none;
        transition: border-color 0.12s ease;
    }
    .form-input:focus {
        border-color: var(--accent);
    }
    "###
}

/// Helper function to render a clean, consistent top header across internal subsystem pages.
fn render_internal_header(active: &str, title: &str) -> String {
    let pages = [
        ("newtab", "Start", "norva://newtab"),
        ("history", "History", "norva://history"),
        ("bookmarks", "Bookmarks", "norva://bookmarks"),
        ("downloads", "Downloads", "norva://downloads"),
        ("passwords", "Passwords", "norva://passwords"),
        ("shields", "Shields", "norva://shields"),
        ("permissions", "Permissions", "norva://permissions"),
        ("cookies", "Storage", "norva://cookies"),
        ("settings", "Settings", "norva://settings"),
    ];

    let mut chips = String::new();
    for (id, label, url) in pages {
        let is_active = if id == active { " active" } else { "" };
        chips.push_str(&format!(
            r###"<a href="{}" class="chip{}">{}</a>"###,
            url, is_active, label
        ));
    }

    format!(
        r###"<header class="internal-header">
        <div class="internal-header-left">
            <a href="norva://newtab" class="back-link">
                <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>
                <span>Start Page</span>
            </a>
            <h2 class="internal-title">{}</h2>
        </div>
        <div class="internal-nav-chips">
            {}
        </div>
    </header>"###,
        title, chips
    )
}

/// Renders the primary `norva://newtab` executive start page.
pub fn render_newtab_page(
    stats: &ShieldStats,
    _engines: &[SearchEngineConfig],
    default_engine: &SearchEngineConfig,
    secure_mode: bool,
    _cache_mb: f64,
) -> String {
    let mode_pill = if secure_mode {
        r###"<span class="session-badge secure"><span class="dot"></span>Private Mode</span>"###
    } else {
        r###"<span class="session-badge normal"><span class="dot"></span>Hardened Mode</span>"###
    };

    let saved_mb = (stats.estimated_bandwidth_saved_kb as f64) / 1024.0;

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>New Tab | NORVA</title>
    <style>
        {{COMMON_CSS}}

        .nt-canvas {
            max-width: 1040px;
            margin: 0 auto;
            padding: 24px 24px 48px;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
        }

        .nt-topbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 24px;
        }

        .nt-brand {
            display: flex;
            align-items: center;
            gap: 10px;
            text-decoration: none;
            color: inherit;
        }
        .nt-brand-text {
            font-size: 19px;
            font-weight: 700;
            letter-spacing: -0.6px;
            color: #ffffff;
        }

        .nt-nav-links {
            display: flex;
            align-items: center;
            gap: 18px;
        }
        .nt-nav-links a {
            color: var(--text-muted);
            text-decoration: none;
            font-size: 13px;
            font-weight: 500;
            transition: color 0.12s ease;
        }
        .nt-nav-links a:hover {
            color: #ffffff;
        }

        .session-badge {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            padding: 3px 10px;
            border-radius: 20px;
            font-size: 11px;
            font-weight: 500;
            letter-spacing: 0.3px;
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid var(--border-subtle);
            color: var(--text-secondary);
        }
        .session-badge .dot {
            width: 6px;
            height: 6px;
            border-radius: 50%;
            background: #10b981;
        }
        .session-badge.secure .dot {
            background: #818cf8;
        }

        /* Hero: Clock & Search */
        .nt-hero {
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-top: 10px;
            margin-bottom: 40px;
        }
        .nt-clock {
            font-size: 76px;
            font-weight: 200;
            letter-spacing: -3px;
            color: #ffffff;
            line-height: 1;
            margin-bottom: 6px;
            user-select: none;
        }
        .nt-date {
            font-size: 11.5px;
            font-weight: 600;
            letter-spacing: 2px;
            color: var(--text-muted);
            text-transform: uppercase;
            margin-bottom: 32px;
            user-select: none;
        }

        .nt-search-box {
            display: flex;
            align-items: center;
            width: 100%;
            max-width: 620px;
            height: 48px;
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.09);
            border-radius: 24px;
            padding: 0 16px;
            box-shadow: 0 8px 30px rgba(0, 0, 0, 0.4);
            transition: all 0.18s cubic-bezier(0.16, 1, 0.3, 1);
        }
        .nt-search-box:focus-within {
            border-color: rgba(99, 102, 241, 0.6);
            background: rgba(255, 255, 255, 0.06);
            box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2), 0 12px 36px rgba(0, 0, 0, 0.5);
        }
        .nt-search-icon {
            color: var(--text-muted);
            margin-right: 12px;
            display: flex;
            align-items: center;
            flex-shrink: 0;
        }
        .nt-search-input {
            flex: 1;
            background: transparent;
            border: none;
            color: #ffffff;
            font-size: 14.5px;
            font-family: inherit;
            outline: none;
        }
        .nt-search-input::placeholder {
            color: var(--text-muted);
        }
        .nt-submit-btn {
            background: transparent;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            padding: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 50%;
            transition: color 0.12s ease;
        }
        .nt-submit-btn:hover {
            color: #ffffff;
        }

        /* Speed Dials Grid */
        .nt-dials-grid {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 24px;
            flex-wrap: wrap;
            max-width: 680px;
            margin: 0 auto 40px;
        }
        .dial-tile {
            display: flex;
            flex-direction: column;
            align-items: center;
            text-decoration: none;
            color: inherit;
            gap: 8px;
            cursor: pointer;
            transition: transform 0.12s ease;
            width: 68px;
        }
        .dial-tile:hover {
            transform: translateY(-2px);
        }
        .dial-squircle {
            width: 52px;
            height: 52px;
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid var(--border-subtle);
            border-radius: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #d4d4d8;
            transition: all 0.12s ease;
        }
        .dial-tile:hover .dial-squircle {
            background: rgba(255, 255, 255, 0.08);
            border-color: var(--border-hover);
            color: #ffffff;
            box-shadow: 0 6px 18px rgba(0, 0, 0, 0.35);
        }
        .dial-label {
            font-size: 11.5px;
            color: var(--text-secondary);
            font-weight: 500;
            text-align: center;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: 68px;
            transition: color 0.12s ease;
        }
        .dial-tile:hover .dial-label {
            color: #ffffff;
        }

        /* Bottom Status Pill */
        .nt-footer {
            display: flex;
            justify-content: center;
            margin-top: auto;
        }
        .status-pill {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 5px 14px;
            border-radius: 20px;
            background: rgba(255, 255, 255, 0.03);
            border: 1px solid var(--border-subtle);
            color: var(--text-muted);
            text-decoration: none;
            font-size: 11.5px;
            font-weight: 450;
            transition: all 0.12s ease;
        }
        .status-pill:hover {
            background: rgba(255, 255, 255, 0.06);
            border-color: var(--border-hover);
            color: var(--text-secondary);
        }
        .status-dot {
            width: 5px;
            height: 5px;
            border-radius: 50%;
            background: #10b981;
        }
        .status-sep {
            color: rgba(255, 255, 255, 0.12);
        }
    </style>
</head>
<body>
    <div class="nt-canvas">
        <header class="nt-topbar">
            <a href="norva://newtab" class="nt-brand">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                    <polygon points="12 2 2 7 12 12 22 7 12 2"/>
                    <polyline points="2 17 12 22 22 17"/>
                    <polyline points="2 12 12 17 22 12"/>
                </svg>
                <span class="nt-brand-text">norva</span>
            </a>

            <nav class="nt-nav-links">
                <a href="norva://history">History</a>
                <a href="norva://bookmarks">Bookmarks</a>
                <a href="norva://downloads">Downloads</a>
                <a href="norva://passwords">Passwords</a>
                <a href="norva://shields">Shields</a>
                <a href="norva://settings">Settings</a>
            </nav>

            <div>
                {{MODE_PILL}}
            </div>
        </header>

        <main class="nt-hero">
            <div id="nt-clock" class="nt-clock">12:00</div>
            <div id="nt-date" class="nt-date">MONDAY, NOVEMBER 23</div>

            <div class="nt-search-box">
                <div class="nt-search-icon">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
                        <circle cx="11" cy="11" r="8"/>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                    </svg>
                </div>
                <input type="text" id="nt-search-input" class="nt-search-input" placeholder="Search with {{DEFAULT_NAME}} or enter URL..." autocomplete="off" autofocus />
                <button class="nt-submit-btn" id="nt-search-btn" title="Search">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
                        <line x1="5" y1="12" x2="19" y2="12"/>
                        <polyline points="12 5 19 12 12 19"/>
                    </svg>
                </button>
            </div>
        </main>

        <section class="nt-dials-grid">
            <a href="#" class="dial-tile" onclick="navigate('https://duckduckgo.com')">
                <div class="dial-squircle">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
                </div>
                <span class="dial-label">DuckDuckGo</span>
            </a>

            <a href="#" class="dial-tile" onclick="navigate('https://github.com')">
                <div class="dial-squircle">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
                </div>
                <span class="dial-label">GitHub</span>
            </a>

            <a href="#" class="dial-tile" onclick="navigate('https://www.wikipedia.org')">
                <div class="dial-squircle">
                    <span style="font-family: serif; font-weight: 700; font-size: 20px;">W</span>
                </div>
                <span class="dial-label">Wikipedia</span>
            </a>

            <a href="#" class="dial-tile" onclick="navigate('https://news.ycombinator.com')">
                <div class="dial-squircle">
                    <span style="font-weight: 700; font-size: 17px; color: #ff6600;">Y</span>
                </div>
                <span class="dial-label">Hacker News</span>
            </a>

            <a href="#" class="dial-tile" onclick="navigate('https://www.reddit.com')">
                <div class="dial-squircle">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><circle cx="9" cy="10" r="1.5" fill="currentColor"/><circle cx="15" cy="10" r="1.5" fill="currentColor"/><path d="M8 15s1.5 2 4 2 4-2 4-2"/></svg>
                </div>
                <span class="dial-label">Reddit</span>
            </a>

            <a href="#" class="dial-tile" onclick="navigate('https://www.torproject.org')">
                <div class="dial-squircle">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>
                </div>
                <span class="dial-label">Tor Project</span>
            </a>

            <a href="#" class="dial-tile" onclick="navigate('https://www.rust-lang.org')">
                <div class="dial-squircle">
                    <span style="font-weight: 700; font-size: 14px;">Rust</span>
                </div>
                <span class="dial-label">Rust Lang</span>
            </a>

            <a href="norva://shields" class="dial-tile">
                <div class="dial-squircle">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                </div>
                <span class="dial-label">Shields</span>
            </a>
        </section>

        <footer class="nt-footer">
            <a href="norva://shields" class="status-pill">
                <span class="status-dot"></span>
                <span>Protected by Norva Shields</span>
                <span class="status-sep">•</span>
                <span>{{ADS_BLOCKED}} blocked</span>
                <span class="status-sep">•</span>
                <span>{{SAVED_MB}} MB saved</span>
            </a>
        </footer>
    </div>

    <script>
        function updateClock() {
            const clock = document.getElementById("nt-clock");
            const date = document.getElementById("nt-date");
            if (!clock || !date) return;
            const now = new Date();
            clock.textContent = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
            date.textContent = now.toLocaleDateString([], { weekday: 'long', month: 'long', day: 'numeric' }).toUpperCase();
        }
        updateClock();
        setInterval(updateClock, 1000);

        function navigate(url) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "navigate", query: url }));
            } else {
                window.location.href = url;
            }
        }

        const input = document.getElementById("nt-search-input");
        const submitBtn = document.getElementById("nt-search-btn");

        input.addEventListener("keydown", (e) => {
            if (e.key === "Enter") {
                const val = input.value.trim();
                if (val) navigate(val);
            }
        });

        submitBtn.addEventListener("click", () => {
            const val = input.value.trim();
            if (val) navigate(val);
        });
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{MODE_PILL}}", mode_pill)
        .replace("{{DEFAULT_NAME}}", &default_engine.name)
        .replace("{{ADS_BLOCKED}}", &stats.total_all_time.to_string())
        .replace("{{SAVED_MB}}", &format!("{:.1}", saved_mb))
}

/// Renders the `norva://history` view with filtering and purge controls.
pub fn render_history_page(entries: &[HistoryEntry]) -> String {
    let mut rows = String::new();
    if entries.is_empty() {
        rows.push_str(r###"<div style="text-align:center; padding: 48px 20px; color: var(--text-muted);">
            <h3 style="color: var(--text-secondary); font-size: 15px; font-weight: 550;">No Browsing History</h3>
            <p style="margin-top: 4px; font-size: 13px;">Visited pages will appear here. In Private Mode, history is never persisted.</p>
        </div>"###);
    } else {
        for e in entries {
            let engine_badge = if let Some(ref eng) = e.search_engine {
                format!(r###"<span style="background: rgba(99, 102, 241, 0.1); color: #818cf8; border: 1px solid rgba(99, 102, 241, 0.25); padding: 1px 6px; border-radius: 4px; font-size: 11px; font-weight: 550;">{}</span>"###, eng)
            } else {
                String::new()
            };

            let query_info = if let Some(ref q) = e.search_query {
                format!(r###"<span style="color: var(--text-secondary); font-weight: 500;">Search: "{}"</span> • "###, q)
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
                            {}{} • {} visits
                        </div>
                    </div>
                    <button class="btn btn-danger" style="padding: 3px 9px; font-size: 11.5px;" onclick="deleteHistory({})">Delete</button>
                </div>"###,
                e.url, e.title, engine_badge, query_info, e.visit_time, e.visit_count, e.id
            ));
        }
    }

    let header = render_internal_header("history", "History");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>History | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .header-actions {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 20px;
            flex-wrap: wrap;
            gap: 14px;
        }
        .search-box {
            background: #090a0f;
            border: 1px solid var(--border-subtle);
            border-radius: 6px;
            padding: 7px 12px;
            color: #ffffff;
            width: 280px;
            outline: none;
            font-size: 13px;
            font-family: inherit;
        }
        .search-box:focus {
            border-color: var(--border-focus);
        }
        .history-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 16px;
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 8px;
            margin-bottom: 8px;
            transition: all 0.12s ease;
        }
        .history-item:hover {
            background-color: var(--bg-surface-elevated);
            border-color: var(--border-hover);
        }
        .history-main {
            overflow: hidden;
            flex: 1;
            margin-right: 14px;
        }
        .history-title {
            font-size: 13.5px;
            font-weight: 550;
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 3px;
        }
        .history-link {
            color: var(--text-primary);
            text-decoration: none;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
        .history-link:hover {
            color: #818cf8;
        }
        .history-meta {
            font-size: 11.5px;
            color: var(--text-muted);
        }
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div class="header-actions">
            <div>
                <h3 style="font-size: 17px; font-weight: 600; color: #ffffff;">Browsing History</h3>
                <p style="color: var(--text-muted); font-size: 12.5px; margin-top: 2px;">Local SQLite database with zero telemetry</p>
            </div>
            <div style="display: flex; gap: 8px;">
                <input type="text" class="search-box" id="search-hist" placeholder="Search history..." oninput="filterHistory()" />
                <button class="btn btn-danger" onclick="clearAllHistory()">Clear History</button>
            </div>
        </div>

        <div id="history-container">
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
            if (confirm("Clear all browsing history?")) {
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
        .replace("{{HEADER}}", &header)
        .replace("{{ROWS}}", &rows)
}

/// Renders the `norva://shields` view with real-time telemetry and metrics.
pub fn render_shields_page(stats: &ShieldStats, events: &[BlockedEvent]) -> String {
    let mut log_rows = String::new();
    if events.is_empty() {
        log_rows.push_str(r###"<tr><td colspan="4" style="text-align:center; padding: 32px; color: var(--text-muted);">No tracking requests recorded yet. Browse any site to inspect live blocks.</td></tr>"###);
    } else {
        for ev in events {
            let (badge_bg, badge_color) = match ev.category {
                crate::db::shields_db::BlockCategory::Ad => ("rgba(6, 182, 212, 0.1)", "#22d3ee"),
                crate::db::shields_db::BlockCategory::Tracker => ("rgba(244, 63, 94, 0.1)", "#fb7185"),
                crate::db::shields_db::BlockCategory::Fingerprinter => ("rgba(168, 85, 247, 0.1)", "#c084fc"),
                crate::db::shields_db::BlockCategory::Cryptominer => ("rgba(245, 158, 11, 0.1)", "#fbbf24"),
                crate::db::shields_db::BlockCategory::TrackingParam => ("rgba(16, 185, 129, 0.1)", "#34d399"),
            };
            log_rows.push_str(&format!(
                r###"<tr>
                    <td><span style="background: {}; color: {}; padding: 2px 7px; border-radius: 4px; font-weight: 600; font-size: 11px; text-transform: uppercase;">{:?}</span></td>
                    <td style="font-family: monospace; color: var(--text-primary); font-size: 12.5px;">{}</td>
                    <td style="color: var(--text-secondary); font-size: 12.5px;">{}</td>
                    <td style="color: var(--text-muted); font-size: 11.5px;">{}</td>
                </tr>"###,
                badge_bg, badge_color, ev.category, ev.blocked_domain, ev.page_domain, ev.timestamp
            ));
        }
    }

    let saved_mb = (stats.estimated_bandwidth_saved_kb as f64) / 1024.0;
    let header = render_internal_header("shields", "Privacy Shields");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Privacy Shields | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 14px;
            margin-bottom: 24px;
        }
        @media (max-width: 800px) {
            .metrics-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        .metric-box {
            background-color: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 10px;
            padding: 16px;
        }
        .metric-title {
            font-size: 11px;
            color: var(--text-muted);
            text-transform: uppercase;
            letter-spacing: 0.5px;
            font-weight: 600;
            margin-bottom: 6px;
        }
        .metric-val {
            font-size: 24px;
            font-weight: 700;
            color: #ffffff;
        }
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div class="metrics-grid">
            <div class="metric-box">
                <div class="metric-title">Total Interceptions</div>
                <div class="metric-val" style="color: #818cf8;">{{TOTAL_ALL}}</div>
            </div>
            <div class="metric-box">
                <div class="metric-title">Ads Blocked</div>
                <div class="metric-val" style="color: #22d3ee;">{{ADS_COUNT}}</div>
            </div>
            <div class="metric-box">
                <div class="metric-title">Trackers Blocked</div>
                <div class="metric-val" style="color: #fb7185;">{{TRACKERS_COUNT}}</div>
            </div>
            <div class="metric-box">
                <div class="metric-title">Bandwidth Saved</div>
                <div class="metric-val" style="color: #34d399;">{{SAVED_MB}} MB</div>
            </div>
        </div>

        <div class="card">
            <h3 style="font-size: 15px; font-weight: 600; color: #ffffff; margin-bottom: 14px;">Blocked Requests Log</h3>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Category</th>
                            <th>Blocked Target</th>
                            <th>Page Context</th>
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
        .replace("{{HEADER}}", &header)
        .replace("{{TOTAL_ALL}}", &stats.total_all_time.to_string())
        .replace("{{ADS_COUNT}}", &stats.total_ads_blocked.to_string())
        .replace("{{TRACKERS_COUNT}}", &stats.total_trackers_blocked.to_string())
        .replace("{{SAVED_MB}}", &format!("{:.1}", saved_mb))
        .replace("{{LOG_ROWS}}", &log_rows)
}

/// Renders the `norva://permissions` view for device policy management.
pub fn render_permissions_page(permissions: &[SitePermission], audits: &[PermissionAuditLog]) -> String {
    let mut perm_rows = String::new();
    if permissions.is_empty() {
        perm_rows.push_str(r###"<tr><td colspan="5" style="text-align:center; padding: 28px; color: var(--text-muted);">No custom site permissions configured. All origins operate under zero-trust gating.</td></tr>"###);
    } else {
        for p in permissions {
            perm_rows.push_str(&format!(
                r###"<tr>
                    <td style="font-weight: 550; color: #ffffff;">{}</td>
                    <td><span class="policy-badge {:?}">{}</span></td>
                    <td><span class="policy-badge {:?}">{}</span></td>
                    <td><span class="policy-badge {:?}">{}</span></td>
                    <td>
                        <button class="btn btn-danger" style="padding: 3px 8px; font-size: 11px;" onclick="resetPermission('{}')">Reset</button>
                    </td>
                </tr>"###,
                p.domain, p.camera, p.camera.as_str(), p.microphone, p.microphone.as_str(), p.location, p.location.as_str(), p.domain
            ));
        }
    }

    let mut audit_rows = String::new();
    if audits.is_empty() {
        audit_rows.push_str(r###"<tr><td colspan="4" style="text-align:center; padding: 24px; color: var(--text-muted);">No device requests recorded.</td></tr>"###);
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
                    <td style="color: var(--text-muted); font-size: 11.5px;">{}</td>
                </tr>"###,
                a.domain, a.permission_type, decision_color, a.decision, a.timestamp
            ));
        }
    }

    let header = render_internal_header("permissions", "Site Permissions");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Permissions | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .policy-badge {
            padding: 2px 7px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
            text-transform: uppercase;
        }
        .policy-badge.Allow {
            background: rgba(16, 185, 129, 0.1);
            color: #34d399;
        }
        .policy-badge.Block {
            background: rgba(244, 63, 94, 0.1);
            color: #fb7185;
        }
        .policy-badge.Ask {
            background: rgba(245, 158, 11, 0.1);
            color: #fbbf24;
        }
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div class="card">
            <h3 style="font-size: 15px; font-weight: 600; color: #ffffff; margin-bottom: 14px;">Domain Policies</h3>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Origin Domain</th>
                            <th>Camera</th>
                            <th>Microphone</th>
                            <th>Location</th>
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
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 14px;">
                <h3 style="font-size: 15px; font-weight: 600; color: #ffffff;">Device Access Audit Log</h3>
                <button class="btn btn-danger" style="font-size: 11.5px; padding: 3px 9px;" onclick="clearAudit()">Clear Audit</button>
            </div>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Domain</th>
                            <th>Device</th>
                            <th>Decision</th>
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
            if (confirm("Clear all hardware audit records?")) {
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
        .replace("{{HEADER}}", &header)
        .replace("{{PERM_ROWS}}", &perm_rows)
        .replace("{{AUDIT_ROWS}}", &audit_rows)
}

/// Renders the `norva://cookies` view for storage management.
pub fn render_cookies_page(cache_mb: f64) -> String {
    let header = render_internal_header("cookies", "Storage & Cookies");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Storage | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .storage-grid {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 16px;
            margin-bottom: 20px;
        }
        @media (max-width: 700px) {
            .storage-grid {
                grid-template-columns: 1fr;
            }
        }
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div class="storage-grid">
            <div class="card">
                <h3 style="font-size: 15px; font-weight: 600; color: #ffffff; margin-bottom: 6px;">Local Disk Cache</h3>
                <p style="font-size: 12.5px; color: var(--text-secondary); margin-bottom: 14px;">Cached web resources in isolated directory</p>
                <div style="font-size: 28px; font-weight: 700; color: #ffffff; margin-bottom: 16px;">{{CACHE_MB}} MB</div>
                <button class="btn btn-danger" onclick="purgeCache()">Purge Disk Cache</button>
            </div>

            <div class="card">
                <h3 style="font-size: 15px; font-weight: 600; color: #ffffff; margin-bottom: 6px;">Storage Isolation Policy</h3>
                <p style="font-size: 12.5px; color: var(--text-secondary); margin-bottom: 14px;">Cookie partitioning across browsing origins</p>
                <div style="display: flex; flex-direction: column; gap: 8px; font-size: 12.5px;">
                    <div style="display: flex; justify-content: space-between;">
                        <span style="color: var(--text-secondary);">Third-Party Cookie Isolation:</span>
                        <span style="color: #34d399; font-weight: 550;">Enforced</span>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span style="color: var(--text-secondary);">Local Storage Partitioning:</span>
                        <span style="color: #34d399; font-weight: 550;">Enforced</span>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span style="color: var(--text-secondary);">Private Ghost Mode:</span>
                        <span style="color: #818cf8; font-weight: 550;">Available</span>
                    </div>
                </div>
            </div>
        </div>
    </main>

    <script>
        function purgeCache() {
            if (confirm("Purge cached resources from disk?")) {
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
        .replace("{{HEADER}}", &header)
        .replace("{{CACHE_MB}}", &format!("{:.1}", cache_mb))
}

/// Renders the `norva://settings` view for browser search engines and profile.
pub fn render_settings_page(
    engines: &[SearchEngineConfig],
    default_engine: &SearchEngineConfig,
    secure_mode: bool,
) -> String {
    let mut engine_rows = String::new();
    for e in engines {
        let is_default_badge = if e.id == default_engine.id {
            r###"<span style="background: rgba(16, 185, 129, 0.1); color: #34d399; padding: 2px 7px; border-radius: 4px; font-size: 11px; font-weight: 600;">DEFAULT</span>"###
        } else {
            r###"<button class="btn" style="padding: 3px 8px; font-size: 11px;" onclick="setDefaultEngine('{}')">Set Default</button>"###
        };

        engine_rows.push_str(&format!(
            r###"<tr>
                <td style="font-weight: 550; color: #ffffff;">{}</td>
                <td style="color: #818cf8; font-family: monospace; font-weight: 600;">{}</td>
                <td style="color: var(--text-secondary); font-size: 12px; font-family: monospace; max-width: 260px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">{}</td>
                <td>{}</td>
            </tr>"###,
            e.name, e.bang, e.query_template, is_default_badge.replace("{}", &e.id)
        ));
    }

    let mode_desc = if secure_mode {
        "Private Session: in-memory operation, zero disk persistence"
    } else {
        "Hardened Privacy: dedicated SQLite vaults with local zero-telemetry storage"
    };

    let header = render_internal_header("settings", "Settings");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Settings | NORVA</title>
    <style>
        {{COMMON_CSS}}
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div class="card" style="margin-bottom: 20px;">
            <h3 style="font-size: 15px; font-weight: 600; color: #ffffff; margin-bottom: 4px;">Session Profile</h3>
            <p style="font-size: 13px; color: var(--text-secondary);">{{MODE_DESC}}</p>
        </div>

        <div class="card">
            <h3 style="font-size: 15px; font-weight: 600; color: #ffffff; margin-bottom: 14px;">Search Providers</h3>
            <div style="overflow-x: auto;">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Search Engine</th>
                            <th>Bang Hotkey</th>
                            <th>Query Template</th>
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
        .replace("{{HEADER}}", &header)
        .replace("{{MODE_DESC}}", mode_desc)
        .replace("{{ENGINE_ROWS}}", &engine_rows)
}

/// Renders the `norva://bookmarks` management view.
pub fn render_bookmarks_page(bookmarks: &[BookmarkEntry]) -> String {
    let mut bookmark_cards = String::new();
    if bookmarks.is_empty() {
        bookmark_cards.push_str(
            r###"<div style="text-align: center; padding: 48px 20px; color: var(--text-muted);">
                <h3 style="font-size: 15px; color: var(--text-secondary); margin-bottom: 4px; font-weight: 550;">No Bookmarks Saved</h3>
                <p style="font-size: 13px;">Save pages with Ctrl+D or the star button in the address bar.</p>
            </div>"###,
        );
    } else {
        bookmark_cards.push_str(r###"<div class="bm-grid">"###);
        for bm in bookmarks {
            bookmark_cards.push_str(&format!(
                r###"<div class="bm-card" data-title="{}" data-url="{}">
                    <div class="bm-card-top">
                        <div class="bm-info">
                            <div class="bm-title">{}</div>
                            <div class="bm-url">{}</div>
                        </div>
                    </div>
                    <div class="bm-footer">
                        <span class="bm-folder-badge">{}</span>
                        <div class="bm-actions">
                            <button class="btn btn-primary" style="padding: 4px 10px; font-size: 11.5px;" onclick="openUrl('{}')">Open</button>
                            <button class="btn btn-danger" style="padding: 4px 9px; font-size: 11.5px;" onclick="deleteBookmark({})">Delete</button>
                        </div>
                    </div>
                </div>"###,
                bm.title.to_lowercase(),
                bm.url.to_lowercase(),
                bm.title,
                bm.url,
                bm.folder,
                bm.url,
                bm.id,
            ));
        }
        bookmark_cards.push_str("</div>");
    }

    let header = render_internal_header("bookmarks", "Bookmarks");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Bookmarks | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .bm-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
            gap: 14px;
        }
        .bm-card {
            background-color: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 10px;
            padding: 14px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            gap: 12px;
            transition: all 0.12s ease;
        }
        .bm-card:hover {
            border-color: var(--border-hover);
            background-color: var(--bg-surface-elevated);
        }
        .bm-card-top {
            display: flex;
            align-items: flex-start;
            gap: 10px;
        }
        .bm-info {
            flex: 1;
            overflow: hidden;
        }
        .bm-title {
            font-size: 13.5px;
            font-weight: 550;
            color: #ffffff;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .bm-url {
            font-size: 11.5px;
            color: var(--text-muted);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            margin-top: 2px;
        }
        .bm-footer {
            display: flex;
            align-items: center;
            justify-content: space-between;
            border-top: 1px solid rgba(255, 255, 255, 0.04);
            padding-top: 8px;
        }
        .bm-folder-badge {
            font-size: 10.5px;
            font-weight: 550;
            color: var(--text-muted);
            background: rgba(255, 255, 255, 0.04);
            border-radius: 4px;
            padding: 2px 6px;
        }
        .bm-actions {
            display: flex;
            gap: 6px;
        }
        .search-bar {
            width: 100%;
            background: #090a0f;
            border: 1px solid var(--border-subtle);
            border-radius: 8px;
            color: #ffffff;
            padding: 8px 14px;
            font-size: 13px;
            font-family: inherit;
            outline: none;
        }
        .search-bar:focus {
            border-color: var(--border-focus);
        }
        .add-form {
            display: none;
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 10px;
            padding: 16px;
            margin-bottom: 20px;
        }
        .form-row {
            display: grid;
            grid-template-columns: 1fr 1fr 120px auto;
            gap: 10px;
            align-items: center;
        }
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; flex-wrap: wrap; gap: 10px;">
            <input type="text" id="bmSearch" class="search-bar" placeholder="Filter bookmarks..." oninput="filterBookmarks()" style="max-width: 380px;">
            <div style="display: flex; gap: 8px;">
                <button class="btn btn-primary" onclick="toggleAddForm()">+ Add Bookmark</button>
                <button class="btn btn-danger" onclick="clearAllBookmarks()">Clear All</button>
            </div>
        </div>

        <div id="addForm" class="add-form">
            <h4 style="margin-bottom: 10px; color: #ffffff; font-size: 14px;">Add Bookmark</h4>
            <div class="form-row">
                <input type="text" id="bmUrl" class="form-input" placeholder="https://example.com">
                <input type="text" id="bmTitle" class="form-input" placeholder="Site Title">
                <input type="text" id="bmFolder" class="form-input" placeholder="Folder" value="Bookmarks">
                <button class="btn btn-primary" onclick="saveBookmark()">Save</button>
            </div>
        </div>

        {{BOOKMARK_CARDS}}
    </main>

    <script>
        function openUrl(url) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "navigate", query: url }));
            } else {
                window.location.href = url;
            }
        }
        function deleteBookmark(id) {
            if (confirm("Delete this bookmark?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "delete_bookmark", id: id }));
                    setTimeout(() => window.location.reload(), 150);
                }
            }
        }
        function clearAllBookmarks() {
            if (confirm("Clear all bookmarks?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "clear_bookmarks" }));
                    setTimeout(() => window.location.reload(), 150);
                }
            }
        }
        function toggleAddForm() {
            const f = document.getElementById("addForm");
            f.style.display = f.style.display === "block" ? "none" : "block";
        }
        function saveBookmark() {
            const url = document.getElementById("bmUrl").value.trim();
            const title = document.getElementById("bmTitle").value.trim() || url;
            const folder = document.getElementById("bmFolder").value.trim() || "Bookmarks";
            if (!url) return;
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "add_bookmark", url: url, title: title, folder: folder }));
                setTimeout(() => window.location.reload(), 150);
            }
        }
        function filterBookmarks() {
            const q = document.getElementById("bmSearch").value.toLowerCase();
            document.querySelectorAll(".bm-card").forEach(c => {
                const title = c.getAttribute("data-title") || "";
                const url = c.getAttribute("data-url") || "";
                c.style.display = (title.includes(q) || url.includes(q)) ? "flex" : "none";
            });
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{HEADER}}", &header)
        .replace("{{BOOKMARK_CARDS}}", &bookmark_cards)
}

/// Renders the `norva://downloads` management view.
pub fn render_downloads_page(downloads: &[DownloadEntry]) -> String {
    let mut download_rows = String::new();
    if downloads.is_empty() {
        download_rows.push_str(
            r###"<div style="text-align: center; padding: 48px 20px; color: var(--text-muted);">
                <h3 style="font-size: 15px; color: var(--text-secondary); margin-bottom: 4px; font-weight: 550;">No Downloads</h3>
                <p style="font-size: 13px;">Downloaded files will appear here with zero telemetry.</p>
            </div>"###,
        );
    } else {
        download_rows.push_str(r###"<div class="dl-list">"###);
        for dl in downloads {
            let size_str = if dl.file_size_bytes > 1024 * 1024 {
                format!("{:.2} MB", dl.file_size_bytes as f64 / (1024.0 * 1024.0))
            } else if dl.file_size_bytes > 1024 {
                format!("{:.1} KB", dl.file_size_bytes as f64 / 1024.0)
            } else {
                format!("{} B", dl.file_size_bytes)
            };

            download_rows.push_str(&format!(
                r###"<div class="dl-item">
                    <div class="dl-info">
                        <div class="dl-filename">{}</div>
                        <div class="dl-sub">{} • {} • {}</div>
                    </div>
                    <div class="dl-status">{}</div>
                    <div class="dl-actions">
                        <button class="btn btn-danger" style="padding: 3px 9px; font-size: 11.5px;" onclick="deleteDownload({})">Delete</button>
                    </div>
                </div>"###,
                dl.filename,
                size_str,
                dl.created_at.split('T').next().unwrap_or(""),
                dl.url,
                dl.status.to_uppercase(),
                dl.id
            ));
        }
        download_rows.push_str("</div>");
    }

    let header = render_internal_header("downloads", "Downloads");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Downloads | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .dl-list {
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .dl-item {
            background-color: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 8px;
            padding: 12px 16px;
            display: flex;
            align-items: center;
            gap: 12px;
            transition: all 0.12s ease;
        }
        .dl-item:hover {
            border-color: var(--border-hover);
        }
        .dl-info {
            flex: 1;
            overflow: hidden;
        }
        .dl-filename {
            font-size: 13.5px;
            font-weight: 550;
            color: #ffffff;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .dl-sub {
            font-size: 11.5px;
            color: var(--text-muted);
            margin-top: 2px;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .dl-status {
            font-size: 10.5px;
            font-weight: 600;
            color: #34d399;
            background: rgba(16, 185, 129, 0.1);
            padding: 2px 7px;
            border-radius: 4px;
            letter-spacing: 0.3px;
        }
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
            <h3 style="font-size: 16px; font-weight: 600; color: #ffffff;">Download Logs</h3>
            <button class="btn btn-danger" onclick="clearDownloads()">Clear Download History</button>
        </div>

        {{DOWNLOAD_ROWS}}
    </main>

    <script>
        function deleteDownload(id) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "delete_download", id: id }));
                setTimeout(() => window.location.reload(), 150);
            }
        }
        function clearDownloads() {
            if (confirm("Clear all download records?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "clear_downloads" }));
                    setTimeout(() => window.location.reload(), 150);
                }
            }
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{HEADER}}", &header)
        .replace("{{DOWNLOAD_ROWS}}", &download_rows)
}

/// Renders the `norva://passwords` secure credentials vault view.
pub fn render_passwords_page(credentials: &[PasswordEntry]) -> String {
    let mut cred_cards = String::new();
    if credentials.is_empty() {
        cred_cards.push_str(
            r###"<div style="text-align: center; padding: 48px 20px; color: var(--text-muted);">
                <h3 style="font-size: 15px; color: var(--text-secondary); margin-bottom: 4px; font-weight: 550;">No Passwords Saved</h3>
                <p style="font-size: 13px;">Save login credentials securely with localized zero-telemetry SQLite storage.</p>
            </div>"###,
        );
    } else {
        cred_cards.push_str(r###"<div class="cred-grid">"###);
        for cred in credentials {
            cred_cards.push_str(&format!(
                r###"<div class="cred-card" data-domain="{}" data-user="{}">
                    <div class="cred-top">
                        <div class="cred-domain">{}</div>
                        <div class="cred-user">{}</div>
                    </div>
                    <div class="cred-pass-box">
                        <input type="password" id="pwd_{}" class="cred-pass-input" value="{}" readonly>
                        <button class="btn" style="padding: 3px 8px; font-size: 11px;" onclick="togglePwd({})">Show</button>
                        <button class="btn btn-primary" style="padding: 3px 9px; font-size: 11px;" onclick="copyPwd('pwd_{}')">Copy</button>
                    </div>
                    <div style="display: flex; justify-content: space-between; align-items: center; border-top: 1px solid rgba(255,255,255,0.04); padding-top: 8px;">
                        <span style="font-size: 11px; color: var(--text-muted);">Updated: {}</span>
                        <button class="btn btn-danger" style="padding: 3px 8px; font-size: 11px;" onclick="deleteCred({})">Delete</button>
                    </div>
                </div>"###,
                cred.domain.to_lowercase(),
                cred.username.to_lowercase(),
                cred.domain,
                cred.username,
                cred.id,
                cred.password,
                cred.id,
                cred.id,
                cred.updated_at.split('T').next().unwrap_or(""),
                cred.id
            ));
        }
        cred_cards.push_str("</div>");
    }

    let header = render_internal_header("passwords", "Passwords");

    let template = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Passwords | NORVA</title>
    <style>
        {{COMMON_CSS}}
        .cred-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 14px;
        }
        .cred-card {
            background-color: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 10px;
            padding: 14px;
            display: flex;
            flex-direction: column;
            gap: 10px;
            transition: all 0.12s ease;
        }
        .cred-card:hover {
            border-color: var(--border-hover);
        }
        .cred-top {
            display: flex;
            flex-direction: column;
            gap: 2px;
        }
        .cred-domain {
            font-size: 14px;
            font-weight: 600;
            color: #ffffff;
        }
        .cred-user {
            font-size: 12px;
            color: var(--text-secondary);
        }
        .cred-pass-box {
            display: flex;
            align-items: center;
            gap: 6px;
            background: #090a0f;
            border: 1px solid var(--border-subtle);
            border-radius: 6px;
            padding: 3px 5px;
        }
        .cred-pass-input {
            background: transparent;
            border: none;
            color: #ffffff;
            font-family: monospace;
            font-size: 13px;
            flex: 1;
            outline: none;
            padding: 3px 5px;
        }
        .add-modal {
            display: none;
            background: var(--bg-surface);
            border: 1px solid var(--border-subtle);
            border-radius: 10px;
            padding: 16px;
            margin-bottom: 20px;
        }
        .form-grid {
            display: grid;
            grid-template-columns: 1fr 1fr 1fr auto;
            gap: 10px;
            align-items: center;
        }
    </style>
</head>
<body>
    {{HEADER}}

    <main class="container">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; flex-wrap: wrap; gap: 10px;">
            <input type="text" id="credSearch" class="form-input" placeholder="Search domain or username..." oninput="filterCreds()" style="max-width: 360px; width: 100%;">
            <div style="display: flex; gap: 8px;">
                <button class="btn btn-primary" onclick="toggleAddModal()">+ Save Login</button>
                <button class="btn btn-danger" onclick="clearAllCreds()">Clear Vault</button>
            </div>
        </div>

        <div id="addModal" class="add-modal">
            <h4 style="margin-bottom: 10px; color: #ffffff; font-size: 14px;">Save Credentials</h4>
            <div class="form-grid">
                <input type="text" id="domainInput" class="form-input" placeholder="example.com">
                <input type="text" id="userInput" class="form-input" placeholder="Username / Email">
                <input type="password" id="passInput" class="form-input" placeholder="Password">
                <button class="btn btn-primary" onclick="saveCred()">Save</button>
            </div>
        </div>

        {{CRED_CARDS}}
    </main>

    <script>
        function toggleAddModal() {
            const m = document.getElementById("addModal");
            m.style.display = m.style.display === "block" ? "none" : "block";
        }
        function togglePwd(id) {
            const inp = document.getElementById("pwd_" + id);
            inp.type = inp.type === "password" ? "text" : "password";
        }
        function copyPwd(id) {
            const inp = document.getElementById(id);
            navigator.clipboard.writeText(inp.value).then(() => {
                alert("Password copied to clipboard.");
            });
        }
        function saveCred() {
            const domain = document.getElementById("domainInput").value.trim();
            const username = document.getElementById("userInput").value.trim();
            const password = document.getElementById("passInput").value;
            if (!domain || !username || !password) return;
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "save_credential", domain: domain, username: username, password: password }));
                setTimeout(() => window.location.reload(), 150);
            }
        }
        function deleteCred(id) {
            if (confirm("Delete this saved credential?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "delete_credential", id: id }));
                    setTimeout(() => window.location.reload(), 150);
                }
            }
        }
        function clearAllCreds() {
            if (confirm("Clear all passwords in vault?")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({ action: "clear_credentials" }));
                    setTimeout(() => window.location.reload(), 150);
                }
            }
        }
        function filterCreds() {
            const q = document.getElementById("credSearch").value.toLowerCase();
            document.querySelectorAll(".cred-card").forEach(c => {
                const d = c.getAttribute("data-domain") || "";
                const u = c.getAttribute("data-user") || "";
                c.style.display = (d.includes(q) || u.includes(q)) ? "flex" : "none";
            });
        }
    </script>
</body>
</html>"###;

    template
        .replace("{{COMMON_CSS}}", get_common_css())
        .replace("{{HEADER}}", &header)
        .replace("{{CRED_CARDS}}", &cred_cards)
}
