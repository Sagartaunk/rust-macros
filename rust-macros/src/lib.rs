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
