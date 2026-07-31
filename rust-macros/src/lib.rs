/// Example of a random macro that returns
/// the integer `4` with no specific type.
///
/// For a macro to be public, it needs to be
/// exported via `#[macro_export]`.
#[macro_export]
macro_rules! four {
    () => {
        1 + 3
    };
}

/// This is a simple macro that multiplies
/// the input it recieves by 10. The input
/// can realistically be of any type that
/// supports multiplication.
///
///
/// The `expr` metavariable catches any expression
/// given to it, and is defined as `name:expr` where
/// `name` can be any arbitrary name and can be used
/// after appending the `$` symbol as in the following
/// example.
#[macro_export]
macro_rules! times_ten {
    ($inp:expr) => {
        // We use the metavariable `imp` by declaring
        // it as `$imp`.
        10 * $inp
    };
}

/// This macro converts a given key value pairs entered in
/// the format `key => value` to a `std::collections::HashMap`.
#[macro_export]
macro_rules! to_hashmap {
    () => {
        std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),*) => {
        // For some reason a `let` must be enclosed inside
        // a conditional statement or `{}`.
        {
            let mut hashmap = std::collections::HashMap::new();
            // Everything inside this block will be repeated
            // as it is followed by a `*` sybmol.
            $(
                hashmap.insert($key,$value);
            )*
            hashmap
        }
    };
}

#[derive(Debug)]
pub struct RemoteArchive {
    pub url: &'static str,
    /// The SHA-256 hash of the file at `url`.
    pub sha256: [u8; 32],
}

#[macro_export]
macro_rules! parse_remote_archive {
    ($vis:vis const $name:ident: RemoteArchive = $cargo_toml_path:literal [
        $(($os:ident, $arch:ident)),* $(,)?
    ];) => {
        $vis const $name: $crate::RemoteArchive = {
            ::toml_const::toml_const!{
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

/// I have stolen this code from google's zerocopy crate.
///
#[doc(hidden)]
pub mod macro_util {
    pub use sha2_const::Sha256;

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
}
