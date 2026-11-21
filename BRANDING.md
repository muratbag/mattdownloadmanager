# Branding: Matt Download Manager

This tree is a source-level rebrand of `anefzaoui/misterdownloadmanager` (commit `b33b12f`) into
**Matt Download Manager**. The abbreviation **MDM** is kept everywhere (it fits Matt Download
Manager too), as are the `mdm-*` crate names and the `MDM_*` environment variables.

Everything user-visible or machine-visible now says Matt Download Manager / `mattdownloadmanager`.
The complete old -> new map, and the contracts a later change must keep in sync, are below.

## 1. Names and identifiers

| Old | New | Where |
|---|---|---|
| `Mister Download Manager` | `Matt Download Manager` | window titles, tray tooltip, menus, dialogs, About, errors, installer, uninstaller entry, shortcuts, version info, remote page, extension |
| `MisterDownloadManager` | `MattDownloadManager` | registry keys/progids, mutex, user agent, release file names, installer macro values |
| `misterdownloadmanager` | `mattdownloadmanager` | exe names, data folder, native messaging host, bundle ids, URLs, `.desktop` id, log/UA strings |
| `mister-download-manager` | `matt-download-manager` | extension store page slugs (AMO, Chrome Web Store) |
| `Mister%20Download%20Manager` | `Matt%20Download%20Manager` | `ms-settings:defaultapps?registeredApp*=` URI |
| `MISTERDOWNLOADMANAGER.EXE` | `MATTDOWNLOADMANAGER.EXE` | uppercase exe name in a win32 test fixture |
| `misterdownloadmanager.com` | `mattdownloadmanager.com` | help/site/extension links (placeholder; see §6) |
| `github.com/anefzaoui/misterdownloadmanager` | `github.com/mattdownloadmanager/mattdownloadmanager` | problem reports, source, licences links (placeholder) |
| `github.com/sponsors/anefzaoui` | `github.com/sponsors/mattdownloadmanager` | About / Help -> Support / extension popup (placeholder) |
| `io.github.anefzaoui.misterdownloadmanager` | `io.github.mattdownloadmanager.mattdownloadmanager` | macOS bundle ids, LaunchAgent labels, Keychain service |

Unchanged on purpose: author/copyright attribution (`Ahmed Nefzaoui`, GPL-3.0-or-later) in
`LICENSE`, `ATTRIBUTION.md`, `Cargo.toml`'s `authors`, the installer's publisher/copyright fields
and the exe's `CompanyName`; dependency git URLs (`anefzaoui/egui`, `anefzaoui/rqbit`, ...); the
`mdm-*` crate names and `MDM_*` variables; the abbreviation MDM; `IDM`/Internet Download Manager
references (a different product).

### Programs (exe names in Task Manager / install folder)

| Binary | Built from |
|---|---|
| `mattdownloadmanager.exe` | `crates/app` (`[[bin]] name = "mattdownloadmanager"`) |
| `mattdownloadmanager-cli.exe` | `crates/cli` (launcher for the app's command line) |
| `mattdownloadmanager-nmhost.exe` | `crates/nmhost` (browser native messaging host) |
| `mattdownloadmanager-mcp.exe` | `crates/mcp` (MCP server) |

Version-info resources (`crates/app/build.rs`): `ProductName` and `FileDescription` are
`Matt Download Manager`; `CompanyName` / `LegalCopyright` keep the original attribution.

### Filesystem, registry, OS integration

| Item | Value |
|---|---|
| Data / config folder | `%APPDATA%\mattdownloadmanager` (`mdm_ipc::data_dir`; `MDM_DATA_DIR` still overrides) |
| Portable data folder | `data\` next to the program (unchanged mechanism) |
| Log | `<data>\mdm.log` (+ `crash.log`); MDM abbreviation kept |
| Registry (HKCU) | `Software\MattDownloadManager` — `InstallLanguage`, `InstallTorrents` |
| Autostart Run value | `HKCU\...\Run` value name `MattDownloadManager`, data `"<app>\mattdownloadmanager.exe" /startup` |
| Single-instance mutex | `MattDownloadManager.Running` |
| `APP_ID` / AppUserModelID / StartupWMClass | `mattdownloadmanager` |
| ProgIDs | `MattDownloadManager.torrent`, `MattDownloadManager.Magnet`, `.Torrent`, `.Magnet` |
| Default apps registration | `HKCU\Software\MattDownloadManager\Capabilities`, `RegisteredApplications` value `Matt Download Manager`, `ms-settings:defaultapps?registeredAppUser=Matt%20Download%20Manager` |
| Credential Manager | target prefix `mattdownloadmanager` (the app's sign-ins) |
| Tray tooltip / main window title | `Matt Download Manager <version>` |
| URI handling | no custom app scheme: `http(s)`, `magnet:` and `.torrent` files as before |

## 2. Installer identity (`tools/installer/mdm.iss`)

| Field | Value |
|---|---|
| AppId | `{{51CA5712-5B18-49EC-BD2C-DEF46253F6D3}` — **new** (an upstream install is never upgraded into this one) |
| AppName / UninstallDisplayName | `Matt Download Manager` |
| AppExe / HostExe | `mattdownloadmanager.exe` / `mattdownloadmanager-nmhost.exe` |
| DefaultDirName | `{autopf}\Matt Download Manager` (with `PrivilegesRequired=lowest`, per-user `%LOCALAPPDATA%\Programs\Matt Download Manager`; no admin rights needed) |
| OutputBaseFilename | `MattDownloadManager-<version>-setup[-<arch>].exe` |
| Start-menu shortcut | `{autoprograms}\Matt Download Manager` (per-user when not elevated) |
| Publisher / support / updates URLs | publisher and copyright kept (`Ahmed Nefzaoui`, GPL attribution); URLs point at the placeholder repository (§6) |
| Uninstall cleanup | removes `com.mattdownloadmanager.host*.json`, `MattDownloadManager.*` ProgIDs, `Software\MattDownloadManager`, `%APPDATA%\mattdownloadmanager` (when the user says so) |

## 3. Browser integration and extension

| Item | Value |
|---|---|
| Native messaging host name | `com.mattdownloadmanager.host` |
| Host manifest file | `com.mattdownloadmanager.host.json` (Chrome/Edge/Brave/Opera/Firefox dirs + HKCU/HKLM `...\NativeMessagingHosts`) |
| Host program | `mattdownloadmanager-nmhost.exe` (`/usr/bin/mattdownloadmanager-nmhost` on Linux) |
| Extension display name / description | `Matt Download Manager` (all 51 `_locales`) |
| Extension ID / manifest `key` | **unchanged** (`ejoleppdfgjdndddcjoehopknmlglojp`) — a locally loaded unpacked extension keeps the same ID, and the host accepts it |
| Firefox gecko id | `integration@mattdownloadmanager.com` (was `integration@misterdownloadmanager.com`) |
| Store page links | `addons.mozilla.org/.../matt-download-manager/`, Chrome Web Store `detail/matt-download-manager/<same id>` — **placeholders**, the store listings do not exist yet |
| Popup strings | all brand strings replaced in `extension/_locales/*/messages.json` and `popup.html` |

## 4. Update feed (contract)

The app updates itself from the project's own server, and from nowhere else. The single place the
address lives:

```text
crates/app/src/help.rs
pub const UPDATE_FEED: &str = "https://idm.teachermatt.com/update.json";
pub const UPDATE_HOST: &str = "idm.teachermatt.com";   // the only host files may come from
```

`MDM_UPDATE_FEED` overrides the feed for the end-to-end tests (its host then becomes the allowed
host). There is no GitHub/upstream URL in the update path any more.

### Feed format (`update.json`, UTF-8 JSON)

```json
{
  "version": "0.5.3",
  "date": "2026-11-20",
  "notes": "https://idm.teachermatt.com/changelog",
  "notes_text": "## What's new\n- ...",
  "page": "https://idm.teachermatt.com/changelog",
  "releases": {
    "x64": {
      "setup":         "https://idm.teachermatt.com/dl/...-setup.exe",
      "setup_sha256":  "<64 hex digits>",
      "portable":      "https://idm.teachermatt.com/dl/...-portable.zip",
      "portable_sha256":"<64 hex digits>",
      "zip":           "https://idm.teachermatt.com/dl/...-x64.zip",
      "zip_sha256":    "<64 hex digits>"
    },
    "arm64": { "setup": "...", "setup_sha256": "...", "portable": "...", "portable_sha256": "...", "zip": "...", "zip_sha256": "..." },
    "x86":   { "setup": "...", "setup_sha256": "...", "portable": "...", "portable_sha256": "...", "zip": "...", "zip_sha256": "..." }
  }
}
```

Required: `version` (numeric dotted, optional leading `v`). Optional: `date` (`YYYY-MM-DD`; without
it the window says "…is available." with no date), `notes` (changelog URL; also the notes text
fallback), `notes_text` (Markdown shown in the "What's new" box), `page` (Download button target;
falls back to `notes`). Optional per processor: `releases.<x64|arm64|x86>.<setup|portable|zip>` +
its `*_sha256`.

Rules enforced by the app (`crates/app/src/help.rs::parse_release`):

* a file URL is taken only when it is `http(s)` and its host is `UPDATE_HOST`; anything else is
  ignored (never `github.com/anefzaoui/*`), and `mdm_tools::update::fetch_feed_file` checks the
  host again before downloading;
* a file is taken only with a 64-hex-digit `*_sha256`; the downloaded bytes are checked against
  that SHA-256 before anything is installed;
* the file's URL path is free; the app saves it under the release name it computes
  (`MattDownloadManager-<version>-setup[-<arch>].exe`, `-portable[-<arch>].zip`, `-<arch>.zip`);
* `version` not newer than the running one -> "You have the latest version of Matt Download
  Manager (x.y.z).";
* offline, 404, non-200, bad JSON, no version -> "Updates could not be checked: …" with Retry;
  nothing changes.

What the site must produce: `update.json` at the URL above, files served directly from
`idm.teachermatt.com` (please avoid a redirect to another host), each file's SHA-256 published in
the feed. There is **no PGP/signed-list step** in this path; integrity relies on HTTPS plus the
feed's hash. The old upstream-key signed-list code (`mdm_tools::update::fetch`,
`signed_by_mdm`) is kept only for the legacy unit tests.

Install behaviour, unchanged: a setup download runs silently with `/relaunch=1`; a portable/plain
zip is unpacked and its files replace the running ones; the app reopens itself with `/updated`.

## 5. Crash and freeze reports

Upload is disabled; no upstream (or other) endpoint is contacted:

* `crates/crash/src/github.rs`: `find()` returns `None` without asking GitHub; `REPO` is the
  placeholder `mattdownloadmanager/mattdownloadmanager`; the crash window's Send button still
  opens a (pre-filled) GitHub issue form *only when the user clicks it* (that is a browser action,
  not an upload). Point `REPO` at a real tracker, or remove the button, if the project has a
  tracker.
* `crates/crash/src/symbols.rs`: `fetch()` downloads nothing and returns an error; reports keep
  the bare addresses and stay local.
* The report itself is still written to `<data>\crashes\*.txt` (and the `.dmp` when kept) and can
  be copied/saved locally from the crash window.

## 6. Links that are placeholders

These are brand-consistent but do not exist yet; replace them when the real pages/accounts exist:

* `https://mattdownloadmanager.com` and its `/help/...` pages (Help -> manual, home page, About's
  Home page link) — `crates/app/src/help.rs::SITE`, extension `homepage_url`, `HELP`, `GET_MDM`.
* `https://github.com/mattdownloadmanager/mattdownloadmanager` (Report a problem, Source code,
  Licenses and credits) — `help.rs::REPO`, `crates/crash/src/github.rs::REPO`, installer
  Publisher/Support/Updates URLs.
* `https://github.com/sponsors/mattdownloadmanager` — `help.rs::SPONSORS`, extension popup.
* Extension store listings: `addons.mozilla.org/firefox/addon/matt-download-manager/` and
  `chromewebstore.google.com/detail/matt-download-manager/<id>` (`crates/ipc/src/extension.rs`).
  The installer ships the extension unpacked for "Load unpacked"/about:debugging, so this is a
  cosmetic link until the listings exist.
* The update feed itself is *not* a placeholder: `idm.teachermatt.com` per the project owner.

## 7. Where to change what

* Update address / allowed host: `crates/app/src/help.rs` `UPDATE_FEED`, `UPDATE_HOST` (nothing
  else names a server).
* Website / repository / sponsors: `help.rs::{SITE, REPO, SPONSORS}`,
  `crates/crash/src/github.rs::REPO`, `extension/popup.js::SPONSORS`, installer URLs.
* Installer identity: `tools/installer/mdm.iss` (`AppId`, `AppName`, `AppExe`, `HostExe`,
  `OutputBaseFilename`, `AppPublisher*`).
* Native messaging host: `crates/ipc/src/browsers.rs::{HOST_NAME, HOST_MANIFEST}` and
  `extension/background.js::HOST` — change them together.
* Extension identity: `extension/manifest*.json` (`key`/`gecko.id`) and
  `crates/ipc/src/extension.rs` (`STORE_ID`, `FIREFOX_ID`, page URLs).
* Product name in the GUI: `i18n/*.tsv` (English source texts are keys) and
  `crates/app/build.rs`; regenerate `i18n/en.tsv` with `python tools/i18n/extract.py`.
* Icon art: `art/Matt Download Manager.png` (build uses `crates/app/assets/icons/app/mdm.ico`).

## 8. Static verification used here

```text
python tools/i18n/extract.py --check          # catalogs in sync with the source texts (exit 0)
rg -i "mister" crates i18n/*.tsv extension tools/installer   # only the signed test fixture below
git diff --stat / git status                  # 337 files changed
```

Expected residuals in the app build path (deliberately kept):

* `crates/tools/tests/fixtures/mdm-SHA256SUMS.txt` (+ `.sig`, and the fixture zip
  `MisterDownloadManager-9.9.9-x64.zip`) — the file's bytes are covered by an upstream release
  key's signature, so it and the asset names in it cannot be renamed without invalidating the
  fixture. Test-only; not shipped.
* `crates/tools/src/update.rs` test module: the same asset names, and the file name inside the
  fixture zip, for the same reason.
