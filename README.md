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

**NORVA** is a high-performance, lightweight, uncompromising privacy web browser built entirely in **Rust**. Engineered from the ground up to neutralize surveillance capitalism, NORVA gives you total sovereignty over your online presence with zero telemetry, dedicated SQLite data vaults for every subsystem, hardware zero-trust control, and seamless support for clearnet and darknet (.onion) search engines.

---

## ⚡ Core Architecture & Technology Stack

| Layer | Technology | Key Capabilities |
| :--- | :--- | :--- |
| **Backend** | **Rust 2024 Edition** | Zero-cost abstractions, memory safety, thread safety, humanly written architectural rationales. |
| **Browser Chrome** | **Native GTK-3 (Custom)** | Authentic desktop browser layout: dedicated multi-tab strip at top, navigation toolbar (Back/Forward/Reload/Home), centered Omnibox with live search engine switcher, Privacy Shield counter, hardware zero-trust indicators, 3-dots menu, and Bookmarks bar. |
| **Web Engine** | **Tao + Wry (WebKitGTK)** | Hardware-accelerated web viewport seamlessly attached below the native chrome, running across both Wayland and X11 without distortion or window handle errors. |
| **Database** | **SQLite (Multi-Vault)** | **A dedicated database for every usecase** with Write-Ahead Logging (WAL) and memory-only Secure Mode. |

---

## 🖥️ Authentic Desktop Browser Interface & Layout

NORVA looks, feels, and functions like an authentic, premium desktop web browser:

1. **Top Tab Bar**:
   - Dedicated tab strip anchored at the very top of the window.
   - Interactive rounded tabs with favicons, document title truncation, and `✕` close buttons.
   - Seamless active tab highlighting physically connected to the toolbar below.
   - Instant `+` button to open new tabs.
   - Dynamic tab title and favicon synchronization as web pages load.
   - Real-time Session Status Pill (`🔒 GHOST MODE` or `🛡️ PRIVACY HARDENED`).

2. **Navigation Toolbar & Centered Omnibox**:
   - Standard browser controls: Back (`‹`), Forward (`›`), Reload (`↻`), and Home (`⌂`).
   - Centered Omnibox (address bar) with rounded capsule styling and glowing focus ring.
   - **Connection Security Badge (`🛡️` / `🔒`)**: Click to inspect HTTPS encryption, hardware isolation, and tracking status.
   - **Search Engine Selector Pill (`🦆 DDG ▼`, `🔍 Google ▼`, `⚡ Bing ▼`, `🟣 Yahoo ▼`, `🧅 Ahmia ▼`)**: Click to switch your default search provider on the fly.
   - **Full URL / Search Entry**: Type any URL, domain name, search query, or bang shortcut (`!g`, `!b`, `!ddg`, `!y`, `!ahmia`) and press `Enter` to navigate.
   - **Bookmark Star Button (`★`)**: One-click bookmarking of active URLs directly into the bookmarks bar.

3. **Status Clusters & Browser Menu**:
   - **Live Privacy Shield Counter (`🛡️ X Blocked`)**: Increments dynamically as ads and trackers are neutralized. Click to open the Shields dashboard popover.
   - **Hardware Indicators Cluster (`📷 🎙️ 📍`)**: Instant popovers showing zero-trust status for webcams, microphones, and geolocation sensors, with quick navigation to `norva://permissions`.
   - **3-Dots Menu (`⋮`)**: Instant access to New Tab, History Vault, Privacy Shields, Site Permissions, Storage & Cache Purge, and Settings.

4. **Bookmarks Bar (Speed Dial)**:
   - Dedicated speed bar below the toolbar providing instant 1-click access to top privacy and search destinations: DuckDuckGo, Google, Bing, Yahoo, Ahmia .onion Tor, Rust Lang, EFF Privacy, Tor Project, History, and Shields.

5. **Start Page (`norva://newtab`)**:
   - Clean, centered hero layout without redundant top navbars.
   - Live Search Box with instant search engine selector tabs.
   - Speed dial cards, real-time privacy metrics grid, hardware status strip, and internal command vault shortcuts.

---

## 🗄️ Multi-Database Architecture: "A New DB for Every Use Case"

Unlike traditional browsers that lump your bookmarks, history, telemetry, and site storage into one giant fragile file, NORVA enforces strict architectural compartmentalization:

1. **`history.db`** (Browsing History Vault):
   - Stores URLs, page titles, search engine queries, timestamps, and visit counts.
   - Indexed for instantaneous chronological lookups and instant substring searches.
   - Deduplicates sequential visits to prevent database bloat.
   - Completely swapped for `:memory:` in Secure Ghost Mode (zero disk writes).

2. **`permissions.db`** (Hardware & Geolocation Arbiter):
   - Stores per-origin permission rules (`Allow`, `Block`, `Ask`) for **Internal & External Cameras**, **Internal & External Microphones**, and **Geolocation sensors**.
   - Maintains an immutable audit log of every device activation attempt.

3. **`privacy_shields.db`** (Anti-Tracking & Adblock Matrix):
   - Tracks neutralized advertising domains, telemetry beacons, fingerprinters, and cryptominers.
   - Stores custom domain rules (blocklist/whitelist) and real-time bandwidth savings metrics.

4. **`settings.db`** (Search Engines & Configuration):
   - Houses the search engine catalog, bang shortcuts, and user preferences.
   - Keeps your search settings safe even if history is purged.

5. **`cache_cookies.db`** (Storage & Cookie Isolation):
   - Controls per-domain storage policies, ephemeral cookie wipes, third-party cookie blocking, and cache cleanup audits.

---

## 🔍 First-Class Search Engine Integration

NORVA provides native, out-of-the-box support for all major search engines, including Tor-style .onion searches:

| Engine | Bang Hotkey | Search URL Template | Description |
| :--- | :--- | :--- | :--- |
| **DuckDuckGo** | `!ddg` | `https://duckduckgo.com/?q={}` | Default privacy search engine (no tracking) |
| **Google Search** | `!g` | `https://www.google.com/search?q={}` | Ubiquitous web search index |
| **Microsoft Bing** | `!b` | `https://www.bing.com/search?q={}` | Microsoft web search index |
| **Yahoo Search** | `!y` | `https://search.yahoo.com/search?p={}` | Yahoo search directory |
| **Ahmia (.onion)** | `!ahmia` / `!onion` | `https://ahmia.fi/search/?q={}` | **Tor .onion hidden services search engine** |
| **Startpage** | `!sp` | `https://www.startpage.com/sp/search?query={}` | Google search results with total privacy protection |
| **Brave Search** | `!brave` | `https://search.brave.com/search?q={}` | Independent privacy search index |

*You can also register custom search providers (such as SearXNG or internal corporate indices) directly from `norva://settings`.*

---

## 📷 Hardware Zero-Trust: Camera, Microphone & Location

- **Internal & External Webcams**: Supports laptop cameras, USB webcams, and DSLR video capture cards.
- **Internal & External Microphones**: Supports built-in laptop mics, USB podcast mics, and XLR audio interfaces.
- **Geolocation Protection**: Sites must request permission before obtaining coordinates; rogue attempts are logged to `permissions.db`.
- **Granular Controls**: Inspect and toggle site policies (`Allow` / `Block` / `Ask`) anytime at `norva://permissions`.

---

## 🛡️ Anti-Tracking & Ad-Blocking Matrix

NORVA operates an active, multi-layered shield that stops trackers before they touch your computer:
1. **Socket-Level Ad Blocking**: Pre-emptively severs outgoing connections to Google DoubleClick, Criteo, Taboola, Outbrain, etc.
2. **Behavioral Telemetry Neutralization**: Blocks Google Analytics, Hotjar, Facebook Pixel, Microsoft Clarity, Yandex Metrika, and telemetry beacons.
3. **URL Parameter Sanitization**: Automatically strips surveillance click tokens (`fbclid`, `gclid`, `utm_source`, `utm_campaign`, `msclkid`, `zanpid`, etc.) when navigating or following links.
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
- Linux with GTK3/4 and WebKitGTK (`webkit2gtk-4.1` or `webkitgtk-6.0`)
- Rust 1.80+ (`cargo`)

### Development Build & Run
```bash
# Standard privacy-hardened run
cargo run

# Launch in ephemeral Secure Ghost Mode
cargo run -- --secure

# Direct navigation to a destination
cargo run -- https://duckduckgo.com
cargo run -- !ahmia privacy tools
```

### Run Integration Tests
```bash
cargo test
```

### Production Build
```bash
cargo build --release
./target/release/norva
```

---

## 🧭 Internal `norva://` Navigation Pages

- `norva://newtab` - Cyber Privacy Dashboard with instant search switcher and live shield stats
- `norva://history` - Searchable History Vault with single-entry deletion and full purge
- `norva://shields` - Anti-Tracking & Ad-Blocker command center and real-time block log
- `norva://permissions` - Camera, Microphone, and Location hardware arbiter
- `norva://cookies` - Cookie partitioner and 1-click Cache Memory Purge
- `norva://settings` - Default search engine selector and custom engine manager
