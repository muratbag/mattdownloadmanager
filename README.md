# Matt Download Manager

[![Latest release](https://img.shields.io/github/v/release/muratbag/mattdownloadmanager?label=release)](https://github.com/muratbag/mattdownloadmanager/releases/latest)
[![Licence](https://img.shields.io/badge/licence-GPL--3.0--or--later-blue)](https://idm.teachermatt.com/licenses.html)
[![Windows](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4)](https://idm.teachermatt.com/download.html)
[![Firefox add-on](https://img.shields.io/amo/v/matt-download-manager?label=Firefox%20add-on)](https://addons.mozilla.org/en-US/firefox/addon/matt-download-manager/)
[![Chrome Web Store](https://img.shields.io/badge/Chrome%20Web%20Store-extension-4285F4)](https://chromewebstore.google.com/detail/matt-download-manager/kiddjkmogppidgcidochfmeibhbjoedi)

**Free, open-source download manager for Windows 10 and 11** — a free alternative to Internet
Download Manager: segmented multi-connection downloads, resume, browser capture, video downloads,
torrents and magnet links, a scheduler, queues, categories, speed limits, portable mode and 86
interface languages. No ads, no trackers, no account, no licence key.

**Download:** https://idm.teachermatt.com — installer (per-user, no administrator rights), portable
ZIP and plain ZIP, with published SHA-256 checksums and automatic updates from
`idm.teachermatt.com`.

**Browser extension:** [Chrome Web Store](https://chromewebstore.google.com/detail/matt-download-manager/kiddjkmogppidgcidochfmeibhbjoedi)
— the installer registers it, so Chrome installs and enables it by itself.

![The download list](https://idm.teachermatt.com/assets/img/shots/index-hero.png)

## What it does

- **Segmented downloads** — several connections per file, resumable, with redirects, cookies,
  authentication, proxies and SOCKS (including the browser's own proxy).
- **Browser capture** — Chrome, Edge, Brave, Opera, Vivaldi and Firefox hand their downloads to the
  program through native messaging; a "Download this video" button appears on pages with a player.
- **Video** — pages are resolved with yt-dlp, which the app **keeps current by itself** at every
  start; MP4 by height plus MP3, subtitles, playlists and channels.
- **Torrents** — magnet links, `.torrent` files, seeding limits, inbound connections when allowed.
- **Everything else** — scheduler, queues, categories, site logins, speed limits, clipboard
  watching, command line, an automation API and an MCP server for AI assistants, 86 languages,
  light/dark themes, right-to-left layouts.
- **Private by design** — no telemetry, no ads, no account; the only network calls are the ones the
  person asks for plus an update check against idm.teachermatt.com.

## Install

1. Download `MattDownloadManager-<version>-setup.exe` from https://idm.teachermatt.com
2. Windows may show *"Windows protected your PC"* while the release is unsigned: **More info →
   Run anyway** (the SHA-256 of every release is published on the download page).
3. The installer is per-user, needs no administrator rights, and registers the browser extension.
   A portable ZIP is available too: unzip anywhere, settings stay in a `data` folder beside it.

## Build

Rust (stable, GNU or MSVC toolchain), CMake and, for the installer, Inno Setup:

```powershell
cargo build --release --no-default-features -p mdm-app -p mdm-nmhost -p mdm-cli -p mdm-mcp
powershell -ExecutionPolicy Bypass -File tools\extension\pack.ps1      # browser extensions
powershell -ExecutionPolicy Bypass -File tools\installer\build.ps1     # installer + zips
```

The long version (toolchain, tests, website, release) is in `WORKPC-SETUP.md`.

## Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by
[SignPath Foundation](https://signpath.org).

Windows releases are built by the GitHub Actions workflow in this repository
(`.github/workflows/build-windows.yml`) from the tagged commit, and the resulting binaries are
signed through SignPath.io. A signature means the file is an automated build of this source code at
that revision.

- **Signed files:** `mattdownloadmanager.exe`, `mattdownloadmanager-nmhost.exe`,
  `mattdownloadmanager-cli.exe`, `mattdownloadmanager-mcp.exe`, and the Inno Setup installer.
- **Team roles:** committers and reviewers are the repository's members (pull requests are
  reviewed); approvers are the repository owners, and every release is approved manually for
  signing.
- **Privacy:** this program will not transfer any information to other networked systems unless
  specifically requested by the user or the person installing or operating it. See
  https://idm.teachermatt.com/privacy.html.

## Licence and attribution

GPL-3.0-or-later. Matt Download Manager is a rebrand of
[Mister Download Manager](https://github.com/anefzaoui/misterdownloadmanager) by Ahmed Nefzaoui
(GPL-3.0); the original copyright and the full attribution are in `LICENSE.txt` and
`ATTRIBUTION.txt`. The name, artwork and website are Matt Download Manager's own.

"Internet Download Manager" and "IDM" are trademarks of their owner; this project is not affiliated
with or endorsed by them.

## Spread the word

This program grows by word of mouth: there is no advertising budget and no paid promotion. If it
saves you time, a star here or a post somewhere helps more than anything else.

- Ready-to-post texts for social media, forums and comments: https://idm.teachermatt.com/spread.html
- Press kit (boilerplate, fact sheet, screenshots, logos): https://idm.teachermatt.com/press.html
- Found a bug or want a feature? Open an issue: https://github.com/muratbag/mattdownloadmanager/issues
