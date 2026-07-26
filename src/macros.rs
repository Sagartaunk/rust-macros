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
