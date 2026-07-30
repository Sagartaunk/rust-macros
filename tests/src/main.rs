use rust_macros::{RemoteArchive, parse_remote_archive};

parse_remote_archive! {
    const REMOTE: RemoteArchive = "../../rust-macros/Cargo.toml" [
        (linux, x86_64),
        (linux, aarch64),
        (macos, x86_64),
        (macos, aarch64),
    ];
}

fn main() {
    println!("{:?}", REMOTE);
}
