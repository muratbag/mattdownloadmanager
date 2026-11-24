//! Live check of the Deno helper (never run in a release): does MDM fetch it, verify it, unpack it
//! and start it? `cargo run -p mdm-tools --example deno_install`.
fn main() {
    let rt = tokio::runtime::Runtime::new().expect("runtime");
    rt.block_on(async {
        let dir = std::env::temp_dir().join("mdm-deno-live");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        let net = mdm_engine::Options::default();
        match mdm_tools::deno::install_latest(&dir, &net, None).await {
            Ok(i) => {
                println!("installed Deno {} at {}", i.version, i.exe.display());
                println!("size: {} MB", i.exe.metadata().map(|m| m.len()).unwrap_or(0) / 1_000_000);
                println!("re-installed (must not download again): {:?}", mdm_tools::deno::installed(&dir).map(|j| j.version));
                match std::process::Command::new(&i.exe).arg("--version").output() {
                    Ok(out) => println!(
                        "deno --version: {}{}",
                        String::from_utf8_lossy(&out.stdout).trim(),
                        String::from_utf8_lossy(&out.stderr).trim()
                    ),
                    Err(e) => println!("could not start Deno: {e}"),
                }
            }
            Err(e) => println!("install failed: {e}"),
        }
    });
}
