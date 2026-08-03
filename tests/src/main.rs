use rust_macros::{parse_remote_archive, read_cargo};

parse_remote_archive! {
    const REMOTE: RemoteArchive = "../../rust-macros/tests/Cargo.toml" [
        (linux, x86_64),
        (linux, aarch64),
        (macos, x86_64),
        (macos, aarch64),
    ];
}

fn main() {
    println!(
        "This is the output of the remote archive macro: \n{:?}",
        REMOTE
    );
    let cargo = read_cargo!();
    println! {"This is what is inside Cargo.toml {}", cargo};
}
