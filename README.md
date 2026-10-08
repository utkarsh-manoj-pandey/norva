# 🛡️ NORVA (Native Open-source Rust-based Virtual Access)

```
  ███╗   ██╗ ██████╗ ██████╗ ██╗   ██╗ █████╗ 
  ████╗  ██║██╔═══██╗██╔══██╗██║   ██║██╔══██╗
  ██╔██╗ ██║██║   ██║██████╔╝██║   ██║███████║
  ██║╚██╗██║██║   ██║██╔══██╗╚██╗ ██╔╝██╔══██║
  ██║ ╚████║╚██████╔╝██║  ██║ ╚████╔╝ ██║  ██║
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═╝  ╚═══╝  ╚═╝  ╚═╝
  Native Open-source Rust-based Virtual Access
  High-Security • Anti-Tracking • Hardware Isolated • Multi-Engine
```

**NORVA** is a high-performance, lightweight, uncompromising privacy web browser built entirely in **Rust**. Engineered from the ground up to neutralize surveillance capitalism, NORVA gives you total sovereignty over your online presence with zero telemetry, dedicated SQLite data vaults for every subsystem, hardware zero-trust control, lock-free atomic telemetry counters, and seamless support for clearnet and darknet (.onion) search engines.

---

## ⚡ Core Architecture & Technology Stack

| Layer | Technology | Key Capabilities |
| :--- | :--- | :--- |
| **Backend** | **Rust 2024 Edition** | Zero-cost abstractions, memory safety, thread safety, humanly written architectural rationales across every module. |
| **Browser Chrome** | **Native GTK-3 (Executive Dark)** | Authentic desktop browser layout: dedicated multi-tab strip at top, navigation toolbar (Back/Forward/Reload/Home), centered Omnibox with live search engine switcher, Privacy Shield counter, hardware zero-trust indicators, 3-dots menu, and Bookmarks bar. |
| **Web Engine** | **Tao + Wry (WebKitGTK)** | Hardware-accelerated web viewport seamlessly attached below the native chrome, running across Wayland and X11 without distortion. |
| **Database** | **SQLite (Multi-Vault)** | **5 dedicated, isolated database files** with Write-Ahead Logging (WAL), 64MB memory caches, busy timeouts, and in-memory Secure Mode. |
| **Performance** | **Lock-Free Atomic Caches** | Zero-allocation subdomain matching, atomic telemetry counters, and asynchronous background cache calculations eliminating browser freezes. |

---

## 🖥️ Executive Desktop Browser Interface & Layout

NORVA features an authentic, executive-grade cyber-dark interface free of childish widgets or amateurish clutter:

1. **Top Tab Bar**:
   - Dedicated tab strip anchored at the very top of the window.
   - Interactive tabs with document title truncation, status glyphs, and `✕` close buttons.
   - Active tab indicator physically connected to the toolbar below.
   - Instant `+` button to open new tabs.
   - Real-time Session Status Pill (`🔒 GHOST MODE` or `● PRIVACY HARDENED`).

2. **Navigation Toolbar & Centered Omnibox**:
   - Standard browser controls: Back (`←`), Forward (`→`), Reload (`↻`), and Home (`⌂`).
   - Centered Omnibox capsule with focus illumination and security badge.
   - **Search Engine Selector Pill (`DDG ▼`, `Google ▼`, `Bing ▼`, `Yahoo ▼`, `Ahmia ▼`, `GitHub ▼`, `Wiki ▼`)**: Click to switch search providers on the fly.
   - **Full URL / Search Entry**: Type any URL, domain name, search query, or bang shortcut (`!g`, `!b`, `!ddg`, `!y`, `!ahmia`, `!gh`, `!w`) and press `Enter` to navigate.
   - **Bookmark Star Button (`☆`)**: One-click bookmarking of active URLs directly into the bookmarks bar.

3. **Status Clusters & Browser Menu**:
   - **Live Privacy Shield Counter (`🛡️ X Blocked`)**: Increments dynamically as ads and trackers are neutralized. Click to open the Shields dashboard popover.
   - **Hardware Indicators Cluster (`📷 🎙️ 📍`)**: Instant popovers showing zero-trust status for webcams, microphones, and geolocation sensors, with quick navigation to `norva://permissions`.
   - **3-Dots Menu (`⋮`)**: Instant access to New Tab, History Vault, Privacy Shields, Site Permissions, Storage & Cache Purge, and Settings.

4. **Bookmarks Bar (Speed Bar)**:
   - Dedicated speed bar below the toolbar providing instant 1-click access to top privacy and search destinations: DuckDuckGo, Ahmia Tor, EFF Privacy, Tor Project, Wikipedia, Rust Lang, Shields Center, and History Vault.

5. **Start Page (`norva://newtab`)**:
   - Clean, centered executive layout.
   - Live Search Box with instant search engine selector tabs and Bang chips.
   - **Real Threat Telemetry Cards**: Threats Neutralized matrix, Zero-Trust Hardware status, Bandwidth Saved metrics, and Storage Quota.
   - **Developer & Privacy Speed Dial Placecards**: Real vector-styled destinations (GitHub, DuckDuckGo, Tor Project, EFF, Rust Lang, Hacker News, Wikipedia, Ahmia, Privacy Guides, Startpage).
   - Instant Vault links to History, Shields, Permissions, Storage, and Settings.

---

## 🗄️ Multi-Database Architecture: "A New DB for Every Use Case"

Unlike traditional browsers that lump bookmarks, history, telemetry, and site storage into one giant fragile file, NORVA enforces strict architectural compartmentalization:

1. **`history.db`** (Browsing History Vault):
   - Stores URLs, page titles, search engine queries, timestamps, and visit counts.
   - Indexed for instantaneous chronological lookups and instant substring searches.
   - Deduplicates sequential visits to prevent database bloat.
   - Swapped for `:memory:` in Secure Ghost Mode (zero disk writes).

2. **`permissions.db`** (Hardware & Geolocation Arbiter):
   - Stores per-origin permission rules (`Allow`, `Block`, `Ask`) for **Internal & External Cameras**, **Internal & External Microphones**, and **Geolocation sensors**.
   - Maintains a tamper-evident audit log of every device activation attempt.

3. **`privacy_shields.db`** (Anti-Tracking & Adblock Matrix):
   - Tracks neutralized advertising domains, telemetry beacons, fingerprinters, and cryptominers.
   - Stores custom domain rules (blocklist/whitelist) and real-time bandwidth savings metrics.
   - Equipped with in-memory lock-free atomic counters for zero-latency retrieval.

4. **`settings.db`** (Search Engines & Configuration):
   - Houses the search engine catalog, bang shortcuts, and user preferences.
   - Keeps search settings safe even if history is purged.

5. **`cache_cookies.db`** (Storage & Cookie Isolation):
   - Controls per-domain storage policies, ephemeral cookie wipes, third-party cookie blocking, and cache cleanup audits.

---

## 🔍 First-Class Search Engine Integration

NORVA provides native, out-of-the-box support for 9 major search engines, including Tor-style .onion searches:

| Engine | Bang Hotkey | Search URL Template | Description |
| :--- | :--- | :--- | :--- |
| **DuckDuckGo** | `!ddg` | `https://duckduckgo.com/?q={}` | Default privacy search engine (no tracking) |
| **Google Search** | `!g` | `https://www.google.com/search?q={}` | Global web search index |
| **Microsoft Bing** | `!b` | `https://www.bing.com/search?q={}` | Microsoft web search index |
| **Yahoo Search** | `!y` | `https://search.yahoo.com/search?p={}` | Yahoo search directory |
| **Ahmia (.onion)** | `!ahmia` / `!onion` | `https://ahmia.fi/search/?q={}` | **Tor .onion hidden services search engine** |
| **Startpage** | `!sp` | `https://www.startpage.com/sp/search?query={}` | Google results served with total European privacy |
| **Brave Search** | `!brave` | `https://search.brave.com/search?q={}` | Independent privacy search index |
| **GitHub** | `!gh` | `https://github.com/search?q={}` | Open-source repositories and code search |
| **Wikipedia** | `!w` | `https://en.wikipedia.org/wiki/Special:Search?search={}` | Free encyclopedia and open knowledge |

*You can also register custom search providers directly from `norva://settings`.*

---

## 📷 Hardware Zero-Trust: Camera, Microphone & Location

- **Internal & External Webcams**: Supports laptop cameras, USB webcams, and DSLR video capture cards.
- **Internal & External Microphones**: Supports built-in laptop mics, USB podcast mics, and XLR audio interfaces.
- **Geolocation Protection**: Sites must request permission before obtaining coordinates; rogue attempts are blocked and logged to `permissions.db`.
- **Granular Controls**: Inspect and toggle site policies (`Allow` / `Block` / `Ask`) anytime at `norva://permissions`.

---

## 🛡️ Anti-Tracking & Ad-Blocking Matrix

NORVA operates an active, multi-layered shield that stops trackers before they touch your computer:
1. **Socket-Level Ad Blocking**: Pre-emptively severs outgoing connections to DoubleClick, Criteo, Taboola, Outbrain, etc. (200+ surveillance endpoints).
2. **Behavioral Telemetry Neutralization**: Blocks Google Analytics, Hotjar, Facebook Pixel, Microsoft Clarity, Yandex Metrika, Datadog, NewRelic, and telemetry beacons.
3. **URL Parameter Sanitization**: Automatically strips 30+ surveillance click tokens (`fbclid`, `gclid`, `gbraid`, `wbraid`, `utm_*`, `msclkid`, `twclid`, `ttclid`, `zanpid`, etc.) when following links.
4. **Canvas & Audio Anti-Fingerprinting**: Injects microscopic deterministic noise into HTML5 Canvas and AudioContext decoders, rendering fingerprinting useless.
5. **Cosmetic Ad Collapsing**: Collapses empty banner containers (`.adsbygoogle`, `[id^="google_ads"]`) without disrupting page layout.

---

## 🔒 Secure Ghost Mode (Zero Trace)

Launch NORVA with `--secure` or `-s` to enter **Secure Ghost Mode**:
- **Ephemeral Databases**: Uses in-memory SQLite (`:memory:`)—nothing is written to physical disk.
- **Zero History**: Navigations are not recorded.
- **Isolated Storage**: Cookies and storage keys evaporate immediately when the window closes.
- **Paranoid Shielding**: Maximum anti-fingerprinting noise and ad filtering.

---

## 💻 Building and Running

### Prerequisites
- Linux with GTK-3 and WebKitGTK (`webkit2gtk-4.1` or `webkitgtk-6.0`)
- Rust 1.80+ (`cargo`)

### Development Build & Run
```bash
# Standard privacy-hardened run
cargo run

# Launch in ephemeral Secure Ghost Mode
cargo run -- --secure

# Launch with initial target URL
cargo run -- https://duckduckgo.com
```

### Running Test Suite
```bash
cargo test
```
