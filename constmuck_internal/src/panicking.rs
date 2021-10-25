#[doc(hidden)]
#[macro_export]
#[cfg(feature = "rust_1_57")]
macro_rules! panic_ {
    (
        $array_hack:block
        $const_panic:block
    ) => (
        $const_panic
    )
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "rust_1_57"))]
macro_rules! panic_ {
    (
        $array_hack:block
        $const_panic:block
    ) => (
        $array_hack
    )
}


