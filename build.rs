//! This build script exposes `os/arch` pairs as
//! well as url's and `sha256` sums written in
//! `Cargo.toml` to macros.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let os = std::env::var("CARGO_CFG_TARGET_OS")?;
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH")?;

    let target_header = format!("[package.metadata.exocrate.{}.{}]", os, arch);

    let toml = std::fs::read_to_string("Cargo.toml")?;
    let mut lines = toml.lines();
    while let Some(line) = lines.next() {
        if line.trim() == target_header {
            let mut url = None;
            let mut sha256 = None;

            for line in &mut lines {
                let line = line.trim();

                if line.starts_with('[') {
                    break;
                }

                if let Some(value) = line.strip_prefix("url=") {
                    url = Some(value.trim_matches('"').to_owned());
                }
                if let Some(value) = line.strip_prefix("sha256 = ") {
                    sha256 = Some(value.trim_matches('"').to_owned());
                }
            }
            println!("cargo:rustc-env=EXOCRATE_URL={}", url.unwrap());
            println!("cargo:rustc-env=EXOCRATE_SHA256={}", sha256.unwrap());
        }
    }

    Ok(())
}
