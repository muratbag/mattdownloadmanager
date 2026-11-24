//! Deno: the JavaScript runtime yt-dlp needs for YouTube's full quality list (#852). MDM fetches
//! it the way it fetches yt-dlp and ffmpeg: the newest release from its GitHub page, verified
//! against the release's own `.zip.sha256sum`, installed next to earlier versions, switched to
//! atomically, never downgraded.
//!
//! Deno is a single program (~41 MB on Windows). It is only ever started by yt-dlp, and only for
//! the players that need JavaScript to decode their links.

use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::{Error, Installed};

/// Deno's newest release, through the API (the release page's downloads are served from another
/// host some networks block, while `api.github.com` answers everywhere MDM's helpers are fetched).
const RELEASE: &str = "https://api.github.com/repos/denoland/deno/releases/latest";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
    size: u64,
}

/// Deno's file name for this system and processor in its releases; None where it publishes none.
pub fn asset() -> Option<&'static str> {
    asset_for(std::env::consts::OS, std::env::consts::ARCH)
}

fn asset_for(os: &str, arch: &str) -> Option<&'static str> {
    Some(match (os, arch) {
        ("windows", "x86_64") => "deno-x86_64-pc-windows-msvc.zip",
        ("windows", "aarch64") => "deno-aarch64-pc-windows-msvc.zip",
        ("windows", "x86") => return None,
        ("macos", "x86_64") => "deno-x86_64-apple-darwin.zip",
        ("macos", "aarch64") => "deno-aarch64-apple-darwin.zip",
        ("linux", "x86_64") => "deno-x86_64-unknown-linux-gnu.zip",
        ("linux", "aarch64") => "deno-aarch64-unknown-linux-gnu.zip",
        _ => return None,
    })
}

/// The program's name in its folder, for yt-dlp's `--js-runtimes deno:<path>`.
fn program_name() -> &'static str {
    if cfg!(windows) { "deno.exe" } else { "deno" }
}

/// MDM's install in `dir` (`dir/deno/current.json`), if it is still there.
pub fn installed(dir: &Path) -> Option<Installed> {
    std::fs::read(dir.join("deno").join("current.json"))
        .ok()
        .and_then(|text| serde_json::from_slice::<Installed>(&text).ok())
        .filter(|i| i.exe.is_file())
}

/// Install the newest Deno into `dir` unless the installed one is as new, downloaded with MDM's
/// engine (`net`), counted into `progress`.
pub async fn install_latest(
    dir: &Path,
    net: &mdm_engine::Options,
    progress: Option<&crate::update::Progress>,
) -> Result<Installed, Error> {
    let Some(asset_name) = asset() else {
        return Err(Error::Missing(
            "Deno build for this system and processor".into(),
        ));
    };
    let client = crate::client(net, RELEASE)?;
    let body = client
        .get(RELEASE)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let release: Release =
        serde_json::from_slice(&body).map_err(|e| Error::BadRelease(e.to_string()))?;
    let version = release.tag_name.trim_start_matches('v').to_owned();
    if let Some(current) = installed(dir)
        && !crate::newer(&version, &current.version)
    {
        return Ok(current);
    }
    let find = |name: &str| release.assets.iter().find(|a| a.name == name);
    let asset = find(asset_name)
        .ok_or_else(|| Error::Missing(format!("Deno {version} ({asset_name})")))?;
    let sums = find(&format!("{asset_name}.sha256sum"))
        .ok_or_else(|| Error::Missing(format!("checksum for {asset_name}")))?;
    let text = client
        .get(&sums.browser_download_url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let expected = sha256_of(&text)
        .ok_or_else(|| Error::Missing(format!("checksum for {asset_name}")))?;
    if asset.size < 10_000_000 {
        return Err(Error::BadRelease(format!("Deno {version} is too small")));
    }
    let root = dir.join("deno");
    std::fs::create_dir_all(&root)?;
    // Named by its checksum: one release's cut-off download resumes, never another's.
    let zip_path = root.join(format!("deno-{}.zip", hex::encode(&expected[..8])));
    crate::fetch_verified(
        &asset.browser_download_url,
        &zip_path,
        expected,
        net,
        progress,
    )
    .await?;
    let res = unpack(&zip_path, &root, &version, &asset.browser_download_url);
    let _ = std::fs::remove_file(&zip_path);
    let exe = res?;
    crate::make_current(&root, Installed { version, exe })
}

/// Deno publishes its checksum the way PowerShell's `Get-FileHash` prints it —
/// `Hash      : CBA84B…` — not as `hash  name`, so [`crate::sum_for`] can't read it.
fn sha256_of(text: &str) -> Option<[u8; 32]> {
    text.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        if !key.trim().eq_ignore_ascii_case("hash") {
            return None;
        }
        hex::decode(value.trim()).ok()?.try_into().ok()
    })
}

/// Unpack `deno(.exe)` from the zip's root into `root/<version>/`.
fn unpack(zip_path: &Path, root: &Path, version: &str, from: &str) -> Result<PathBuf, Error> {
    if version.is_empty() || version.contains(['/', '\\', ':']) || version.starts_with('.') {
        return Err(Error::BadRelease(format!("version {version:?}")));
    }
    let folder = root.join(version);
    std::fs::create_dir_all(&folder)?;
    let mut zip = zip::ZipArchive::new(std::fs::File::open(zip_path)?)
        .map_err(|e| Error::BadRelease(e.to_string()))?;
    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| Error::BadRelease(e.to_string()))?;
        let Some(path) = entry.enclosed_name() else {
            continue;
        };
        // The program alone, wherever the archive keeps it (nothing can climb out: the name is
        // taken, never the path, and only `deno(.exe)` is written).
        if !entry.is_file() {
            continue;
        }
        let Some(name) = path.file_name().map(|n| n.to_owned()) else {
            continue;
        };
        if name != program_name() {
            continue;
        }
        let dest = folder.join(&name);
        let mut out = std::fs::File::create(&dest)?;
        std::io::copy(&mut entry, &mut out)?;
        crate::mark_downloaded(&dest, from);
    }
    let exe = folder.join(program_name());
    if !exe.is_file() {
        return Err(Error::Missing("deno in the build".into()));
    }
    Ok(exe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_program_for_each_system_is_named() {
        assert_eq!(
            asset_for("windows", "x86_64"),
            Some("deno-x86_64-pc-windows-msvc.zip")
        );
        assert_eq!(
            asset_for("windows", "aarch64"),
            Some("deno-aarch64-pc-windows-msvc.zip")
        );
        assert_eq!(
            asset_for("linux", "x86_64"),
            Some("deno-x86_64-unknown-linux-gnu.zip")
        );
        assert_eq!(
            asset_for("macos", "aarch64"),
            Some("deno-aarch64-apple-darwin.zip")
        );
        // 32-bit Windows is not published: the person's own Deno, or none.
        assert_eq!(asset_for("windows", "x86"), None);
        assert_eq!(asset_for("freebsd", "x86_64"), None);
    }

    #[test]
    fn the_checksum_powershell_prints_is_read() {
        let text = "Algorithm : SHA256\nHash      : CBA84B26E0DF5AE7F74FC66B86A95B866872A86292EBD84FA1E959927EDD93BF\nPath      : C:\\a\\deno\\target\\release\\deno.exe\n";
        let sum = sha256_of(text).expect("a checksum");
        assert_eq!(sum[0], 0xCB);
        assert_eq!(sum[31], 0xBF);
        assert!(sha256_of("nothing here").is_none());
    }

    #[test]
    fn a_version_never_climbs_out_of_its_folder() {
        let tmp = std::env::temp_dir().join("mdm-deno-test");
        let _ = std::fs::create_dir_all(&tmp);
        for bad in ["", "../escape", "a/b", r"a\b", "C:", ".hidden"] {
            assert!(unpack(&tmp.join("missing.zip"), &tmp, bad, "test").is_err());
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
