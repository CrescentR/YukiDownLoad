<div align="center">

<img src="src-tauri/icons/128x128@2x.png" width="128" height="128" alt="Yuki logo" />

# Yuki · 雪迹

**A lightweight, focused, cross-platform desktop video downloader**
<br/>
Analyze first, choose next, download last — built on Tauri 2 + Rust, powered by yt-dlp and FFmpeg

[![Release](https://img.shields.io/github/v/release/CrescentR/YukiDownLoad?include_prereleases&style=flat-square&color=6c8cff&label=release)](https://github.com/CrescentR/YukiDownLoad/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/CrescentR/YukiDownLoad/yuki-ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/CrescentR/YukiDownLoad/actions/workflows/yuki-ci.yml)
[![Platforms](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-8a8f98?style=flat-square)](#-installation)
[![Tauri](https://img.shields.io/badge/Tauri-2-24c8db?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-stable-dea584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Unlicense-4caf50?style=flat-square)](LICENSE)

[简体中文](README.md) · **English**

[Installation](#-installation) · [Quick Start](#-quick-start) · [User Guide](#-user-guide) · [FAQ](#-faq) · [Development](#️-development)

</div>

---

## ✨ Overview

**Yuki (雪迹, "snow trail")** is a native desktop downloader built around [yt-dlp](https://github.com/yt-dlp/yt-dlp) and [FFmpeg](https://ffmpeg.org). When you paste a link, Yuki first reads the video's **real** title, thumbnail, duration and full format list — resolution, codec, frame rate and estimated size for every track — so you can decide exactly what to download.

- 🪶 **Lightweight** — Tailwind CSS 4 + native ES modules on the front end, no heavy UI framework. The download queue, process supervision and dependency management are all written in Rust.
- 📦 **Ready out of the box** — no Python and no admin rights required. On first launch, download and verify yt-dlp and FFmpeg from inside the app with one click.
- 🌐 **Wide site support** — inherits yt-dlp's coverage of 1000+ sites, including YouTube, Bilibili and Douyin.
- 🎨 **Polished** — dark and light themes, instant switching between English and Simplified Chinese, bundled self-hosted fonts.

## 🧩 Features

<table>
<tr>
<td width="50%" valign="top">

### 🔍 Analyze before download
Shows the real format list (`format_id`, codec, FPS, HDR, estimated size) and prefers direct HTTPS streams. Picking a video track automatically pairs it with the best audio track.

</td>
<td width="50%" valign="top">

### 📚 Batch & playlists
Paste many links at once, one per line. Download a whole playlist or pick entries with range syntax like `1-10,15`.

</td>
</tr>
<tr>
<td valign="top">

### 🍪 Multi-site cookies
Read from Chrome / Edge / Firefox / Brave / Opera / Vivaldi / Safari, import `cookies.txt`, or simply paste the `Cookie` request header — no file needed.

</td>
<td valign="top">

### ⏯️ Queue & resume
Configurable concurrency with automatic queuing. Pause keeps partial fragments and resume continues from there; cancel cleans up the whole `ffmpeg` process tree.

</td>
</tr>
<tr>
<td valign="top">

### 🌍 Proxy & transfer control
HTTP / HTTPS / SOCKS5 proxy (shared by downloads and dependency updates), rate limit, fragment concurrency and retry count.

</td>
<td valign="top">

### 🩺 Live logs & diagnostics
A details drawer shows the live log and the full (redacted) command line. Common errors — login required, 403, missing FFmpeg / JS runtime, timeouts — come with actionable fixes.

</td>
</tr>
<tr>
<td valign="top">

### 🛡️ Managed runtimes
yt-dlp and FFmpeg downloads are SHA-256 verified, test-run and rolled back on failure. Check for and install the latest upstream versions in-app.

</td>
<td valign="top">

### 🎵 Audio & extras
Extract FLAC / MP3 / M4A / Opus audio; write metadata, embed thumbnails and subtitles (including auto-generated ones), save `info.json`. Ships with a Douyin compatibility plugin.

</td>
</tr>
</table>

## 📥 Installation

Grab the installer for your platform from the **[Releases page](https://github.com/CrescentR/YukiDownLoad/releases)**:

| Platform | Package | Notes |
| --- | --- | --- |
| 🪟 **Windows** 10 / 11 (x64) | `Yuki_x.y.z_x64-setup.exe` | NSIS installer, per-user install, no admin rights needed |
| 🍎 **macOS** (Apple Silicon) | `Yuki_x.y.z_aarch64.dmg` | M1 / M2 / M3 / M4 Macs |
| 🍎 **macOS** (Intel) | `Yuki_x.y.z_x64.dmg` | Intel Macs |
| 🐧 **Linux** (x64) | `.AppImage` / `.deb` | AppImage runs anywhere; `.deb` for Debian / Ubuntu |

> [!IMPORTANT]
> The installers are currently **not code-signed**, so your OS may block the first launch. Follow the steps below to allow it.

<details>
<summary><b>🪟 Windows: "Windows protected your PC"</b></summary>

<br/>

In the SmartScreen dialog, click **More info → Run anyway**.

</details>

<details>
<summary><b>🍎 macOS: "is damaged and can't be opened" or "unidentified developer"</b></summary>

<br/>

1. Open the DMG and drag **Yuki** into the Applications folder.
2. Remove the quarantine flag in Terminal:

   ```bash
   xattr -cr /Applications/Yuki.app
   ```

3. Open Yuki again. Alternatively, right-click Yuki in Finder → **Open**, or go to **System Settings → Privacy & Security** and click **Open Anyway**.

</details>

<details>
<summary><b>🐧 Linux: run the AppImage / install the deb</b></summary>

<br/>

```bash
chmod +x Yuki_*.AppImage
./Yuki_*.AppImage
```

```bash
sudo apt install ./Yuki_*.deb
```

</details>

## 🚀 Quick Start

Your first download takes three steps:

```text
 ① Install runtimes         ② Paste & analyze          ③ Pick a format & download
 Settings → Runtime    →    Paste link → Analyze  →   Choose quality → Download
```

1. **Install runtimes** — after the first launch, open **Settings → Runtime** and install **yt-dlp** and **FFmpeg**. Yuki downloads them from official sources, verifies and test-runs them, all without admin rights.
   > On macOS, install FFmpeg yourself (`brew install ffmpeg` is recommended); Yuki detects it automatically.
2. **Paste a link** — copy a video URL and paste it into the input box at the top (or turn on *Watch clipboard for links* in Settings), then click **Analyze**.
3. **Pick & download** — review the title, thumbnail and all formats in the analysis panel, select the quality you want and click **Download this format**. When it finishes, use *Open file* or *Show in folder*.

## 📖 User Guide

### 1. Batch downloads

Paste **one link per line** into the input box. Yuki shows how many links it recognized and adds each to the queue after analysis.

### 2. Playlists

After pasting a playlist link, the analysis panel lists its entries (the preview reads at most the first 500 entries; this does not limit what is actually downloaded):

- Click **Download whole playlist** to get every entry;
- Set a range in **Settings → Download → Items**, e.g. `1-10,15` for entries 1–10 and 15;
- **Settings → Download → Playlist** lets you choose *Ask each time / Decide from the link / Only the single video / Download the whole list*.

> [!TIP]
> If a YouTube video URL contains `list=` and you only want that one video, set the playlist mode to *Only the single video*.

### 3. Audio only

Choose an **Audio only** track in the analysis panel, or set **Settings → Download → Default quality** to *Audio only* and pick FLAC / MP3 / M4A / Opus under *Audio format*. Audio conversion requires FFmpeg.

### 4. Subtitles, thumbnails & metadata

Toggle these under **Settings → Download**:

| Option | What it does |
| --- | --- |
| Write metadata | Writes title, uploader and other info into the file |
| Embed thumbnail | Embeds the video thumbnail (requires FFmpeg) |
| Download & embed subtitles | Comma-separated languages, e.g. `en,zh-Hans` |
| Include auto-generated subtitles | Also fetches the platform's automatic captions |
| Save info.json | Saves the full video metadata |
| Download archive | Records downloaded videos to skip duplicates |

### 5. Cookies (members-only content / higher quality)

Some content requires a signed-in account — members-only or age-restricted videos, or Bilibili's 1080p60 / 4K / HDR. Open **Settings → Network & sign-in** and pick one method:

<details open>
<summary><b>Method A: paste the Cookie header (recommended, most reliable)</b></summary>

<br/>

1. Sign in to the site in your browser, press <kbd>F12</kbd> → **Network**, reload the page and select any request sent to that site.
2. Under **Request Headers**, copy the whole `Cookie` line or just its value.
   > Do not copy `Set-Cookie` from the response headers.
3. In Yuki, choose the site the cookie belongs to: **YouTube** for YouTube; for anything else choose **Other website (custom)** and enter the actual source domain, e.g. `https://www.bilibili.com`.
4. Paste, save, then analyze again.

Yuki writes a local cookie file, keeps cookies for other domains, and turns off browser reading automatically. The raw content is never stored in settings or task arguments, and the input is cleared after saving. You can also paste full Netscape `cookies.txt` content, which carries its own domains and ignores the site option.

</details>

<details>
<summary><b>Method B: read from a browser</b></summary>

<br/>

Pick your signed-in browser under *Cookies from browser*.

> [!WARNING]
> On Windows, **fully quit Chrome / Edge (including background processes)** before reading their cookies. If decryption still fails, use Method A, Method C or Firefox instead. When reading fails, **Disable browser cookies and retry** in the error panel turns it off and re-analyzes public content in one click.

</details>

<details>
<summary><b>Method C: use a cookies.txt file</b></summary>

<br/>

Export a Netscape-format `cookies.txt` with a browser extension and select it under *Cookie file*. Once set, only the file is used and the browser is no longer read.

</details>

> [!CAUTION]
> Cookies are equivalent to your login credentials. **Never share cookies or screenshots that show cookie values.**

### 6. Proxy & network

Under **Settings → Network & sign-in**:

- **Proxy** — e.g. `http://127.0.0.1:7890` or `socks5://127.0.0.1:1080`; shared by downloads and dependency updates. Not needed in TUN mode.
- **Speed limit** — e.g. `2M` for 2 MB/s; leave empty for unlimited.
- **Concurrent fragments** — speeds up sites that serve fragments; too high may get you throttled.
- **Enable JavaScript runtime** — YouTube needs [Deno](https://deno.com) or [Node.js](https://nodejs.org) to solve signatures. Yuki auto-detects `deno` / `node` / `bun` / `qjs` on your system.

### 7. Managing tasks

- **Filter** — view *All / Downloading / Queued / Paused / Completed*.
- **Pause & resume** — pausing keeps partial fragments; resuming continues where it left off.
- **Details** — click any task to open the side drawer with the live log, the full command line (one-click copy, secrets redacted), output path and exit code.
- **Concurrent downloads** — adjust in **Settings → Download**; extra tasks wait in the queue.

### 8. Appearance & language

**Settings → Appearance** switches between the dark (Matte Graphite) and light (Warm Stone) themes, and between English / Simplified Chinese / System.

## ❓ FAQ

<details>
<summary><b><code>HTTP Error 403: Forbidden</code> while downloading (YouTube)</b></summary>

<br/>

A successful page analysis doesn't guarantee the media requests will work. Check that `youtube.com` and `*.googlevideo.com` go through the same exit, try switching proxy nodes and analyze again. Only set up YouTube cookies if the log also mentions sign-in or bot verification.

</details>

<details>
<summary><b>"Sign in required" / only low quality is available</b></summary>

<br/>

Set up cookies from an account with the right access (see [Cookies](#5-cookies-members-only-content--higher-quality)) and analyze again. For example, Bilibili 4K, HDR and 1080p60 may require a premium membership. Quality presets cannot unlock formats the site doesn't return.

</details>

<details>
<summary><b>FFmpeg missing / can't merge audio and video</b></summary>

<br/>

Windows / Linux: install FFmpeg with one click in **Settings → Runtime**. macOS: run `brew install ffmpeg` and restart Yuki.

</details>

<details>
<summary><b>YouTube says a JavaScript runtime is missing</b></summary>

<br/>

Install [Deno](https://deno.com) or [Node.js](https://nodejs.org), make sure it's on your `PATH`, and turn on *Enable JavaScript runtime* in Settings.

</details>

<details>
<summary><b>Analysis is slow / a site stopped working after it changed</b></summary>

<br/>

Check for and install yt-dlp updates in **Settings → Runtime** — site redesigns usually need a newer yt-dlp. If playlist analysis takes over 120 seconds, the engine log stays on screen so you can tell network waits from list processing.

</details>

<details>
<summary><b>Can I use my own yt-dlp?</b></summary>

<br/>

Yes. Yuki looks for yt-dlp in this order and never writes to global system locations:

1. The path in the `YUKI_YTDLP_BIN` environment variable;
2. The app-managed directory (Windows: `%APPDATA%\dev.yuki.downloader\bin\yt-dlp.exe`; Linux / macOS: `bin/yt-dlp` under the app config directory);
3. `yt-dlp` on the system `PATH` (installed via winget, scoop, brew, apt, pipx, …);
4. Python module fallback: `PYTHON` → `python` → `python3` → `py`, launched as `python -m yt_dlp`.

</details>

---

## 🛠️ Development

### Requirements

- **Node.js** 22+
- **Rust** stable toolchain
- Platform build dependencies — on Linux: `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `patchelf`, etc. (see [`yuki-ci.yml`](.github/workflows/yuki-ci.yml))

### Local development

```bash
npm install            # Install front-end and Tauri CLI dependencies
npm run dev            # Run the full desktop app (real Rust download engine)
npm run dev:frontend   # Front-end preview only at http://127.0.0.1:1420 (mock data)
```

### Tests & checks

```bash
npm run check:release                                   # Version consistency & license resources
npm run build:frontend                                  # Build front-end assets
cargo test --lib --manifest-path src-tauri/Cargo.toml   # Rust unit & integration tests
```

### Packaging

```bash
npm run build                      # Build installers for the current platform
npm run build -- --bundles nsis    # Windows NSIS installer only
```

Output goes to `src-tauri/target/release/bundle/` (NSIS / MSI / DMG / AppImage / deb).

### Headless self-checks

The compiled `yuki` binary includes headless diagnostic flags (use `YUKI_CONFIG_DIR` for an isolated config directory):

```bash
yuki --check-updates     # Query the latest upstream yt-dlp and FFmpeg (respects proxy settings)
yuki --install-ytdlp     # Download, verify and install the managed yt-dlp
yuki --install-ffmpeg    # Download, verify and install the managed FFmpeg
yuki --diagnose <url>    # Run one diagnostic download with real arguments and print the command line
```

### Dev scripts

```bash
npm run icons   # Reproducibly generate all app icons and favicons from vector drawing code

# With dev:frontend running, run UI regression checks over the Chrome DevTools Protocol
node devscripts/inspect-ui.mjs http://127.0.0.1:1420/ qa-native.png 1180 800 dark devscripts/verify-native-ui.js
node devscripts/inspect-ui.mjs http://127.0.0.1:1420/ qa-settings.png 1180 800 dark devscripts/verify-settings.js
```

<details>
<summary><b>📂 Project layout</b></summary>

```text
.
├── index.html                  # Front-end entry (sync theme preset to avoid a white flash)
├── vite.config.js              # Vite + Tailwind CSS 4 config
├── public/                     # Static icons and the theme-init script
├── src/                        # Native ES module front end
│   ├── main.js                 # State store, lifecycle, Tauri events with polling fallback
│   ├── styles.css              # Design system and dark / light theme tokens
│   ├── core/
│   │   ├── api.js              # Tauri IPC bridge and browser-preview mock data
│   │   ├── i18n.js             # Chinese / English strings
│   │   └── util.js             # Formatting, link extraction, validation, DOM helpers
│   └── ui/
│       ├── shell.js            # App shell (custom title bar, sidebar, status bar)
│       ├── preview.js          # Analysis dialog (formats and playlist ranges)
│       ├── tasklist.js         # Task list (DOM reused per task ID)
│       ├── drawer.js           # Task drawer (incremental log, redacted command)
│       ├── settings.js         # Settings dialog (draft editing, managed runtimes)
│       ├── actions.js          # Global event delegation and interaction flow
│       ├── select.js           # Custom select component
│       ├── icons.js            # Inline SVG icons
│       └── toasts.js           # Toast notifications
├── src-tauri/                  # Tauri 2 / Rust back end
│   ├── tauri.conf.json         # Window, CSP and bundle config
│   ├── capabilities/           # Tauri 2 least-privilege permissions
│   ├── resources/              # Bundled resources (Douyin compatibility plugin)
│   └── src/
│       ├── lib.rs              # Command registration, concurrent queue, CLI self-checks
│       ├── download.rs         # Arguments, log tailing, progress parsing, process-tree cleanup
│       ├── probe.rs            # Media analysis, encoding tolerance, format ranking
│       ├── cookies.rs          # Browser cookies and multi-site cookies.txt merging
│       ├── runtime.rs          # yt-dlp and JS runtime discovery
│       ├── deps.rs             # FFmpeg detection and package-manager guidance
│       ├── update.rs           # Online updates, resumable download, SHA-256, atomic swap
│       └── settings.rs         # Settings and download history persistence
├── scripts/check-release.mjs   # Release metadata and license checks
├── devscripts/                 # Icon generation and CDP UI regression scripts
└── docs/                       # Research notes and fix log
```

</details>

<details>
<summary><b>⚙️ Engineering notes</b></summary>

- **Structured progress protocol** — instead of fragile regexes over human-readable output, `--progress-template` and `--print` emit machine-readable fields for accurate progress, bytes, speed and ETA across separate DASH audio/video streams and subtitle stages.
- **Targeted DOM updates** — the shell mounts once, task cards are reused in place by `task.id`, progress events are throttled and merged, and logs are appended incrementally, so the UI stays smooth with many fast downloads.
- **Windows encoding & handle stability** — child output goes to managed temp files tailed by byte offset on a background thread, avoiding pipe-handle `OSError: [Errno 22]` and crashes on non-UTF-8 code pages (e.g. `cp936`) with CJK titles.
- **Whole process-tree cleanup** — pause and cancel terminate spawned `ffmpeg` children too (`taskkill /T /F` on Windows, process-group signals on Unix), so no orphan keeps the output file locked.

</details>

### CI & releases

- **[`yuki-ci.yml`](.github/workflows/yuki-ci.yml)** — release metadata checks, front-end build and Rust tests on Windows, Ubuntu and macOS.
- **[`yuki-release.yml`](.github/workflows/yuki-release.yml)** — on manual dispatch or a `yuki-v*` tag, builds Windows NSIS, Linux AppImage / deb and macOS (Apple Silicon & Intel) DMG. Tags also create a **draft Release** with the installers attached. Manual runs can tick `devtools` for a diagnostic build with the Web Inspector enabled.

## 🤝 Contributing

Issues and pull requests are welcome! Before submitting, please make sure:

1. `npm run check:release` and `cargo test --lib --manifest-path src-tauri/Cargo.toml` pass;
2. New UI strings are provided in both Chinese and English (see [`src/core/i18n.js`](src/core/i18n.js));
3. Code follows [`.editorconfig`](.editorconfig).

## ⚖️ Disclaimer

Yuki is only a graphical front end for yt-dlp and FFmpeg; it does not provide or host any media. Download only content you have the right to keep, and follow your local laws and each site's terms of service.

## 📄 License

Yuki itself is released into the public domain under the [**Unlicense**](LICENSE). External tools downloaded at runtime (the official standalone yt-dlp is GPLv3+, the managed FFmpeg is LGPLv2.1+ with its bundled licenses), as well as bundled fonts and dependencies, keep their original licenses — see [**THIRD_PARTY_NOTICES.md**](THIRD_PARTY_NOTICES.md).

## 🙏 Acknowledgements

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) — the download engine
- [FFmpeg](https://ffmpeg.org) and [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) — audio and video processing
- [Tauri](https://tauri.app) — lightweight cross-platform desktop framework
- [Inter](https://rsms.me/inter/), [Noto Sans SC](https://fonts.google.com/noto/specimen/Noto+Sans+SC), [JetBrains Mono](https://www.jetbrains.com/lp/mono/) — UI fonts

<div align="center">

<sub>Made with ❄️ by Yuki contributors</sub>

</div>
