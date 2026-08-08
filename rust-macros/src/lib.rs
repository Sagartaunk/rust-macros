//! Most of the code this crate contains is stolen
//! from google's zerocopy crate.
#[derive(Debug)]
pub struct RemoteArchive {
    pub url: &'static str,
    /// The SHA-256 hash of the file at `url`.
    pub sha256: [u8; 32],
}

#[macro_export]
macro_rules! test_find_line {
    () => {{
        const MANIFEST: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));

        const PREFIX: &str = "[package.metadata.exocrate.";

        let offset = $crate::macro_util::find_line(
            MANIFEST,
            std::env::consts::OS,
            std::env::consts::ARCH,
            PREFIX,
        )
        .expect("Unsupported Platform");

        MANIFEST[..offset].bytes().filter(|&b| b == b'\n').count() + 1
    }};
}

#[macro_export]
macro_rules! parse_remote_archive {
    ($vis:vis const $name:ident: RemoteArchive;) => {
        $vis const $name: $crate::RemoteArchive = {
            const MANIFEST: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
            const PREFIX: &str = "[package.metadata.exocrate.";

            let Some(header) = $crate::macro_util::find_line(
                MANIFEST,
                ::std::env::consts::OS,
                ::std::env::consts::ARCH,
                PREFIX,
            ) else {
                panic!("unsupported platform")
            };

            // Advance past the header line to the first field line.
            let bytes = MANIFEST.as_bytes();
            let mut body_start = header;
            while body_start < bytes.len() && bytes[body_start] != b'\n' {
                body_start += 1;
            }
            if body_start < bytes.len() {
                body_start += 1;
            }

            let Some(url) = $crate::macro_util::find_field(MANIFEST, body_start, "url") else {
                panic!("missing `url` field")
            };
            let Some(sha256_str) =
                $crate::macro_util::find_field(MANIFEST, body_start, "sha256")
            else {
                panic!("missing `sha256` field")
            };
            let Some(sha256) = $crate::macro_util::decode_hex(sha256_str) else {
                panic!("invalid sha256")
            };

            $crate::RemoteArchive { sha256, url }
        };
    };
}

pub use toml_const;

/// This is the legacy renamed parse macro.
#[macro_export]
macro_rules! parse_remote_archive_legacy {
    ($vis:vis const $name:ident: RemoteArchive = $cargo_toml_path:literal [
        $(($os:ident, $arch:ident)),* $(,)?
    ];) => {
        $vis const $name: $crate::RemoteArchive = {
            $crate::toml_const::toml_const!{
                const MANIFEST: $cargo_toml_path;
            }

            let config = {
                use std::env::consts::*;
                use $crate::macro_util::pack;

                // NOTE: Rust doesn't support checking `&str`s for equality in a
                // `const` context. We work around that limitation by packing
                // their bytes into `u128`s, which can be compared.
                //
                // FIXME(#3410): How can we detect if os/arch pairs have been added to
                // `Cargo.toml` without being added to the macro invocation?
                match (pack(OS), pack(ARCH)) {
                    $(
                        (os, arch) if os == pack(stringify!($os)) && arch == pack(stringify!($arch)) => {
                            MANIFEST.package.metadata.exocrate.$os.$arch
                        }
                    )*
                    _ => panic!("unsupported platform"),
                }
            };

            let Some(sha256) = $crate::macro_util::decode_hex(config.sha256) else {
                panic!("invalid sha256")
            };
            $crate::RemoteArchive {
                sha256,
                url: config.url,
            }
        };
    }
}

#[macro_export]
macro_rules! macro2 {
    ($manifest:literal) => {{
        $crate::toml_const::toml_const! {
            const MANIFEST: $manifest;
        }

        let _ = MANIFEST.package.metadata.exocrate.map();

        panic!()
    }};
}

/// I have stolen this code from google's zerocopy crate.
///
#[doc(hidden)]
pub mod macro_util {
    pub use crate::RemoteArchive;
    pub use sha2_const::Sha256;

    #[doc(hidden)]
    pub struct ParsedRemoteArchive<'a> {
        pub url: &'a str,
        pub sha256: &'a str,
    }

    /// Packs the bytes of `s` into a `u128`.
    ///
    /// # Panics
    ///
    /// Panics if `s.as_bytes().len() > 16`.
    pub const fn pack(s: &str) -> u128 {
        let b = s.as_bytes();
        assert!(b.len() <= 16, "slice too large to pack into u128");

        let mut res = 0u128;
        let mut i = 0;
        while i < b.len() {
            res |= (b[i] as u128) << (i * 8);
            i += 1;
        }
        res
    }

    /// Decodes a hexadecimal string into its byte representation.
    pub const fn decode_hex(s: &str) -> Option<[u8; 32]> {
        let bytes = s.as_bytes();
        if bytes.len() != 64 {
            return None;
        }
        let mut res = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            let (h, l) = (bytes[i * 2], bytes[i * 2 + 1]);
            let h_nib = match decode_nibble(h) {
                Some(n) => n,
                None => return None,
            };
            let l_nib = match decode_nibble(l) {
                Some(n) => n,
                None => return None,
            };
            res[i] = (h_nib << 4) | l_nib;
            i += 1;
        }
        Some(res)
    }

    const fn decode_nibble(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    }

    pub const fn encode_hex<const N: usize, const M: usize>(bytes: &[u8; N]) -> [u8; M] {
        assert!(
            M == N * 2,
            "Output buffer must be exactly twice the input length"
        );

        let mut res = [0u8; M];
        const HEX_TABLE: &[u8; 16] = b"0123456789abcdef";

        let mut i = 0;
        while i < N {
            res[i * 2] = HEX_TABLE[(bytes[i] >> 4) as usize];
            res[i * 2 + 1] = HEX_TABLE[(bytes[i] & 0x0f) as usize];
            i += 1;
        }
        res
    }

    /// Returns `true` if the bytes in `data` starting at `offset` exactly match
    /// `to_search`.
    ///
    /// If `offset + to_search.len()` would exceed the bounds of `data`, this
    /// function returns `false`.
    pub const fn bytes_eq_at(data: &[u8], offset: usize, to_search: &[u8]) -> bool {
        if offset + to_search.len() > data.len() {
            return false;
        }

        let mut i = 0;
        while i < to_search.len() {
            if data[offset + i] != to_search[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    /// Searches `manifest` for a metadata section whose header matches
    /// `prefix`, `os`, and `arch`.
    /// The expected format is `[metadata.exocrate.manifest.{os}.{arch}]`
    pub const fn find_line(manifest: &str, os: &str, arch: &str, prefix: &str) -> Option<usize> {
        let bytes = manifest.as_bytes();
        let os = os.as_bytes();
        let arch = arch.as_bytes();
        let prefix = prefix.as_bytes();

        let mut i = 0;

        while i < bytes.len() {
            // Only consider the beginning of lines.
            if i != 0 && bytes[i - 1] != b'\n' {
                i += 1;
                continue;
            }

            if !bytes_eq_at(bytes, i, prefix) {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
                continue;
            }

            let mut pos = i + prefix.len();

            if !bytes_eq_at(bytes, pos, os) {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
                continue;
            }
            pos += os.len();

            if pos >= bytes.len() || bytes[pos] != b'.' {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
                continue;
            }
            pos += 1;

            if !bytes_eq_at(bytes, pos, arch) {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
                continue;
            }
            pos += arch.len();

            if pos < bytes.len() && bytes[pos] == b']' {
                return Some(i);
            }

            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
        }

        None
    }

    /// This function finds the `value` for a passed `key` in the passed
    /// `manifest` after starting at `start` offset. It  returns a `None`
    /// variant if it hits a `[` or an `EOF` before finding the key.
    ///
    /// Note: Expected format for key/value pair is:
    /// `Key = "Value"` do mind the spaces otherwise it will return `None`.
    pub const fn find_field<'a>(manifest: &'a str, start: usize, key: &str) -> Option<&'a str> {
        let bytes = manifest.as_bytes();
        let key = key.as_bytes();
        let mut i = start;
        while i < bytes.len() {
            if bytes[i] == b'[' {
                return None;
            }
            if bytes_eq_at(bytes, i, key) {
                let mut pos = i + key.len();
                // expect exactly " = \"" (space, equals, space, quote)
                if pos + 4 <= bytes.len()
                    && bytes[pos] == b' '
                    && bytes[pos + 1] == b'='
                    && bytes[pos + 2] == b' '
                    && bytes[pos + 3] == b'"'
                {
                    pos += 4;
                    let value_start = pos;
                    let mut value_end = value_start;
                    while value_end < bytes.len() && bytes[value_end] != b'"' {
                        value_end += 1;
                    }
                    if value_end < bytes.len() {
                        let (_, rest) = manifest.split_at(value_start);
                        let (value, _) = rest.split_at(value_end - value_start);
                        return Some(value);
                    }
                }
            }
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
        }
        None
    }
}
