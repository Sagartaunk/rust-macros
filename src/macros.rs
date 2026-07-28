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
    ($vis:vis const $name:ident: RemoteArchive;) => {
        $vis const $name: $crate::RemoteArchive = {
            let Some(sha256) =
                $crate::macro_util::decode_hex(env!("EXOCRATE_SHA256"))
            else {
                panic!("invalid sha256");
            };

            $crate::RemoteArchive {
                sha256,
                url: env!("EXOCRATE_URL"),
            }
        };
    };
}
