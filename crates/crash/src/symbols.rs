//! The release's symbols, which make a report readable: the Breakpad `.sym` files CI makes for
//! each release (too large to ship: the Windows debug info alone is 128 MB), published as one zip
//! per system beside the release's downloads. This build publishes no such release and must not
//! fetch anything from another project's: [`fetch`] downloads nothing, and a report is made
//! readable from symbols already on this computer (if any).

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::github::REPO;

/// This build's system and processor in the zip's name: one zip per download that has programs
/// of its own (release.yml, the platform tools/installer/symbols.py is given).
pub fn platform() -> &'static str {
    let arm = cfg!(target_arch = "aarch64");
    if cfg!(windows) {
        if arm {
            "windows-arm64"
        } else if cfg!(target_arch = "x86") {
            "windows-x86"
        } else {
            "windows-x64"
        }
    } else if cfg!(target_os = "macos") {
        if arm { "macos-arm64" } else { "macos-x86_64" }
    } else if arm {
        "linux-arm64"
    } else {
        "linux-x86_64"
    }
}

/// The release asset holding `version`'s symbols for this system.
pub fn asset_name(version: &str) -> String {
    format!("MattDownloadManager-{version}-symbols-{}.zip", platform())
}

pub fn asset_url(version: &str) -> String {
    format!(
        "https://github.com/{REPO}/releases/download/v{version}/{}",
        asset_name(version)
    )
}

/// Where `version`'s symbols are kept once fetched (a Breakpad symbol store:
/// `<debug file>/<id>/<name>.sym`).
pub fn store(data: &Path, version: &str) -> PathBuf {
    data.join("symbols").join(version)
}

/// Fetched already (any .sym under the store)?
pub fn have(data: &Path, version: &str) -> bool {
    fn any_sym(dir: &Path, depth: usize) -> bool {
        let Ok(rd) = std::fs::read_dir(dir) else {
            return false;
        };
        rd.flatten().any(|e| {
            let p = e.path();
            if p.is_dir() {
                depth > 0 && any_sym(&p, depth - 1)
            } else {
                p.extension().is_some_and(|x| x == "sym")
            }
        })
    }
    any_sym(&store(data, version), 3)
}

/// Fetch `version`'s symbols into the store with MDM's own engine (`net`); `progress(done,
/// total)` as it downloads. A download cut off resumes next time. Earlier versions' symbols are
/// removed (only the running version's are useful).
///
/// This build has no release of its own to fetch them from, so nothing is downloaded: the report
/// keeps the bare addresses the stacks show, and stays on this computer.
pub async fn fetch(
    data: &Path,
    version: &str,
    net: mdm_engine::Options,
    progress: &(dyn Fn(u64, Option<u64>) + Send + Sync),
) -> Result<(), String> {
    let _ = (data, version, net, progress);
    Err("this build publishes no symbols".to_owned())
}

/// Unzip `bytes` into `dir`, every entry's path checked to stay inside it.
pub fn extract(bytes: &[u8], dir: &Path) -> Result<(), String> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|e| e.to_string())?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
        let Some(rel) = entry.enclosed_name() else {
            continue;
        };
        let out = dir.join(rel);
        if entry.is_dir() {
            std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
            continue;
        }
        if let Some(p) = out.parent() {
            std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
        }
        let mut buf = Vec::new();
        entry.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        std::fs::write(&out, buf).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_follow_the_release() {
        assert!(asset_name("0.2.3").starts_with("MattDownloadManager-0.2.3-symbols-"));
        assert!(asset_url("0.2.3").contains("/releases/download/v0.2.3/"));
    }

    #[test]
    fn a_zip_lands_in_the_store_and_counts_as_had() {
        let tmp = tempfile::tempdir().unwrap();
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut w = zip::ZipWriter::new(&mut buf);
            let o = zip::write::SimpleFileOptions::default();
            w.start_file("app.pdb/ABC1/app.sym", o).unwrap();
            std::io::Write::write_all(&mut w, b"MODULE windows x86_64 ABC1 app.pdb\n").unwrap();
            // An entry trying to leave the folder is skipped.
            w.start_file("../evil.sym", o).unwrap();
            w.finish().unwrap();
        }
        extract(buf.get_ref(), &store(tmp.path(), "0.2.3")).unwrap();
        assert!(have(tmp.path(), "0.2.3"));
        assert!(!tmp.path().join("symbols").join("evil.sym").exists());
    }
}
