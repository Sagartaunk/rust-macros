pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
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

#[macro_export]
macro_rules! parse_remote_archive {
    ($vis:vis const $name:ident: RemoteArchive) => {
        $vis const $name: $crate::RemoteArchive = {
            const MANIFEST: &str =
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $cargo_toml_path));
            use std::env::consts::*;
            const PREFIX: &str = "[package.metadata.exocrate.";

            // Get `Sha256`/`Url` for the target Os/Arch pair.
            let Some(offset) = $crate::macro_util::find_line(
                MANIFEST,
                OS,
                ARCH,
                PREFIX
            ) else { panic!("Unsupported Platform")};
            let Some(config) = $crate::macro_util::parse_remote_archive(
                MANIFEST,
                offset,
            ) else {
                panic!("invalid exocrate metadata");
            };
            let Some(sha256) = $crate::macro_util::decode_hex(config.sha256) else {
                panic!("invalid sha256");
            };
            $crate::RemoteArchive {
                url: config.url,
                sha256,
            }
        };
    }
}
