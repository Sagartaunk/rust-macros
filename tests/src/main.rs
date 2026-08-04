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

fn main() {
    println!("legacy url: {}", legacy::HOST_ARCHIVE.url);
    println!("scan url:   {}", scan::REMOTE.url);

    assert_eq!(legacy::HOST_ARCHIVE.url, scan::REMOTE.url);
    assert_eq!(legacy::HOST_ARCHIVE.sha256, scan::REMOTE.sha256);

    println!("outputs match");
}
