mod scan {
    use rust_macros::{RemoteArchive, parse_remote_archive};

    parse_remote_archive! {
        pub const REMOTE: RemoteArchive;
    }
}

mod legacy {
    use rust_macros::{RemoteArchive, parse_remote_archive_legacy};

    parse_remote_archive_legacy! {
        pub const HOST_ARCHIVE: RemoteArchive = "Cargo.toml" [
            (linux, x86_64),
            (linux, aarch64),
            (macos, x86_64),
            (macos, aarch64),
        ];
    }
}

// Macro B: does "something" with the two &str values — here, sums their byte lengths
macro_rules! macro_b {
    ($os:expr, $arch:expr) => {
        $os.len() + $arch.len()
    };
}

// const fn C: takes OS and ARCH as &'static str, uses macro B, returns result
const fn c(os: &'static str, arch: &'static str) -> usize {
    macro_b!(os, arch)
}

// Macro A: pulls std::env::consts::OS/ARCH and calls C
macro_rules! macro_a {
    () => {{
        const RESULT: usize = c(std::env::consts::OS, std::env::consts::ARCH);
        RESULT
    }};
}

fn main() {
    println!("legacy url: {}", legacy::HOST_ARCHIVE.url);
    println!("scan url:   {}", scan::REMOTE.url);

    assert_eq!(legacy::HOST_ARCHIVE.url, scan::REMOTE.url);
    assert_eq!(legacy::HOST_ARCHIVE.sha256, scan::REMOTE.sha256);

    println!("outputs match");

    let out = macro_a!();
    println!("OS = {}", std::env::consts::OS);
    println!("ARCH = {}", std::env::consts::ARCH);
    println!("combined length = {}", out);
}
