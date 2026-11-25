//! Windows version information and icon for `mattdownloadmanager-nmhost.exe`: the same metadata the app carries.
//! A program without any version resource is what antivirus heuristics distrust first (the
//! helper was flagged by ESET on a fresh install, 2026-11-17); this is what can be fixed in code.
//! Code signing (a certificate) is the real trust signal and is applied by the release packaging.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let icon = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../app/assets/icons/app/mdm.ico")
        .canonicalize()
        .expect("app icon");
    println!("cargo:rerun-if-changed={}", icon.display());
    let mut res = winresource::WindowsResource::new();
    res.set_icon(icon.to_str().expect("icon path"))
        .set("ProductName", "Matt Download Manager")
        .set("FileDescription", "Matt Download Manager browser helper")
        .set("CompanyName", "Teacher Matt")
        .set(
            "LegalCopyright",
            "Copyright (C) 2026 Teacher Matt. GPL-3.0-or-later.",
        )
        .set("OriginalFilename", "mattdownloadmanager-nmhost.exe")
        .set("InternalName", "mattdownloadmanager-nmhost.exe")
        .set("FileVersion", env!("CARGO_PKG_VERSION"))
        .set("ProductVersion", env!("CARGO_PKG_VERSION"));
    if let Err(e) = res.compile() {
        // A missing resource compiler must not break the build.
        println!("cargo:warning=version info not embedded: {e}");
    }
}
