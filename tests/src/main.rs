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
use rust_macros::{RemoteArchive, macro2};

const ARCHIVE: RemoteArchive = macro2!("Cargo.toml");

use const_format::formatcp;

macro_rules! macro_one {
    ($name:ident) => {
        const $name: &str = {
            use std::env::consts::{ARCH, OS};
            formatcp!("This is a string {} {}", OS, ARCH)
        };
    };
}

macro_one!(OOS);

fn main() {
    println!("legacy url: {}", legacy::HOST_ARCHIVE.url);
    println!("scan url:   {}", scan::REMOTE.url);

    assert_eq!(legacy::HOST_ARCHIVE.url, scan::REMOTE.url);
    assert_eq!(legacy::HOST_ARCHIVE.sha256, scan::REMOTE.sha256);

    println!("outputs match");

    println!("{}", OOS);

    println!("{}", ARCHIVE.url);
}
