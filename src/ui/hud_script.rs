// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/ui/hud_script.rs
// Description: Injected Cyber Chrome HUD toolbar providing ubiquitous browser controls.
// ============================================================================

/// Returns the JavaScript code injected into documents to render the NORVA Floating Cyber HUD.
///
/// I have written this injected HUD because when users navigate to external websites like
/// Wikipedia or DuckDuckGo, they still need instantaneous access to their address bar,
/// search engine switcher, privacy shield tracker counts, and hardware indicators without
/// sacrificing screen real estate.
pub fn get_hud_injection_script() -> &'static str {
    r###"
    (function() {
        if (window.__norva_hud_injected) return;
        window.__norva_hud_injected = true;

        // Skip injection on internal norva:// pages which already have full UI chrome
        if (window.location.protocol === 'norva:') return;

        const hudContainer = document.createElement('div');
        hudContainer.id = 'norva-cyber-hud-root';

        const shadow = hudContainer.attachShadow({ mode: 'open' });

        const style = document.createElement('style');
        style.textContent = `
            :host {
                all: initial;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', sans-serif;
                z-index: 2147483647;
                position: fixed;
                top: 8px;
                left: 50%;
                transform: translateX(-50%);
                display: block;
                transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
            }
            .hud-bar {
                background: rgba(10, 11, 16, 0.92);
                backdrop-filter: blur(20px) saturate(180%);
                -webkit-backdrop-filter: blur(20px) saturate(180%);
                border: 1px solid rgba(99, 102, 241, 0.3);
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.7), 0 0 15px rgba(99, 102, 241, 0.15);
                border-radius: 9999px;
                padding: 6px 14px;
                display: flex;
                align-items: center;
                gap: 10px;
                color: #f8fafc;
                font-size: 13px;
                cursor: default;
                user-select: none;
                transition: all 0.25s ease;
            }
            .hud-bar:hover {
                border-color: rgba(99, 102, 241, 0.6);
                box-shadow: 0 10px 40px rgba(0, 0, 0, 0.85), 0 0 25px rgba(99, 102, 241, 0.25);
            }
            .brand-badge {
                display: flex;
                align-items: center;
                gap: 6px;
                font-weight: 800;
                letter-spacing: 0.8px;
                color: #ffffff;
                cursor: pointer;
            }
            .btn {
                background: rgba(255, 255, 255, 0.08);
                border: 1px solid rgba(255, 255, 255, 0.12);
                color: #f8fafc;
                border-radius: 6px;
                padding: 4px 8px;
                font-size: 12px;
                cursor: pointer;
                transition: all 0.15s ease;
                display: flex;
                align-items: center;
                gap: 4px;
            }
            .btn:hover {
                background: rgba(99, 102, 241, 0.25);
                border-color: rgba(99, 102, 241, 0.5);
                color: #ffffff;
            }
            .omnibox-input {
                background: rgba(0, 0, 0, 0.45);
                border: 1px solid rgba(255, 255, 255, 0.15);
                border-radius: 9999px;
                padding: 5px 14px;
                color: #ffffff;
                font-size: 12px;
                width: 240px;
                outline: none;
                transition: all 0.2s ease;
            }
            .omnibox-input:focus {
                width: 380px;
                border-color: #6366f1;
                box-shadow: 0 0 10px rgba(99, 102, 241, 0.3);
            }
            .shield-badge {
                background: rgba(16, 185, 129, 0.15);
                border: 1px solid rgba(16, 185, 129, 0.3);
                color: #10b981;
                padding: 3px 8px;
                border-radius: 9999px;
                font-size: 11px;
                font-weight: 600;
                display: flex;
                align-items: center;
                gap: 4px;
                cursor: pointer;
            }
        `;

        const hud = document.createElement('div');
        hud.className = 'hud-bar';
        hud.innerHTML = `
            <div class="brand-badge" id="hud-home" title="NORVA Start Page">
                NORVA
            </div>
            <button class="btn" id="hud-back" title="Back">‹</button>
            <button class="btn" id="hud-fwd" title="Forward">›</button>
            <button class="btn" id="hud-reload" title="Reload">↻</button>
            <input type="text" class="omnibox-input" id="hud-omnibox" placeholder="Search (!g, !b, !ddg, !ahmia) or URL..." />
            <div class="shield-badge" id="hud-shields" title="NORVA Privacy Shields & Anti-Tracking">
                <span>🛡️</span>
                <span>Shields</span>
            </div>
            <button class="btn" id="hud-history" title="History">📜</button>
        `;

        shadow.appendChild(style);
        shadow.appendChild(hud);

        if (document.body) {
            document.body.appendChild(hudContainer);
        } else {
            window.addEventListener('DOMContentLoaded', () => document.body.appendChild(hudContainer));
        }

        const input = shadow.getElementById('hud-omnibox');
        input.value = window.location.href;

        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const val = input.value.trim();
                if (val && window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(JSON.stringify({
                        action: "navigate",
                        query: val
                    }));
                }
            }
        });

        shadow.getElementById('hud-home').addEventListener('click', () => {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "navigate", query: "norva://newtab" }));
            }
        });

        shadow.getElementById('hud-back').addEventListener('click', () => {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "back" }));
            } else {
                window.history.back();
            }
        });

        shadow.getElementById('hud-fwd').addEventListener('click', () => {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "forward" }));
            } else {
                window.history.forward();
            }
        });

        shadow.getElementById('hud-reload').addEventListener('click', () => {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "reload" }));
            } else {
                window.location.reload();
            }
        });

        shadow.getElementById('hud-shields').addEventListener('click', () => {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "navigate", query: "norva://shields" }));
            }
        });

        shadow.getElementById('hud-history').addEventListener('click', () => {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "navigate", query: "norva://history" }));
            }
        });
    })();
    "###
}
