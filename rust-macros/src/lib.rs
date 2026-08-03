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
/// I have stolen this code from google's zerocopy crate.
///
#[doc(hidden)]
pub mod macro_util {
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
}
