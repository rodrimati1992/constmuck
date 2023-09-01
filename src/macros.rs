#[cfg(feature = "debug_checks")]
macro_rules! __check_size {
    ($from:ty, $to:ty) => {
        if core::mem::size_of::<$from>() != core::mem::size_of::<$to>() {
            crate::__priv_utils::transmute_unequal_size_panic(
                core::mem::size_of::<$from>(),
                core::mem::size_of::<$to>(),
            )
        }
    };
}

#[cfg(not(feature = "debug_checks"))]
macro_rules! __check_size {
    ($from:ty, $to:ty) => {};
}

#[cfg(feature = "debug_checks")]
macro_rules! __check_same_alignment {
    ($from:ty, $to:ty) => {
        if core::mem::align_of::<$from>() != core::mem::align_of::<$to>() {
            crate::__priv_utils::transmute_unequal_align_panic(
                core::mem::align_of::<$from>(),
                core::mem::align_of::<$to>(),
            )
        }
    };
}

#[cfg(not(feature = "debug_checks"))]
macro_rules! __check_same_alignment {
    ($from:ty, $to:ty) => {};
}

// Defined this to transmute generic types,
// since `core::mem::transmute` can't transmute between generic (non-concrete) types.
//
// this is unsafe to use for the same reason that `transmute::<$from, $to>` is,
// the types might not be compatible.
macro_rules! __priv_transmute {
    ($from:ty, $to:ty, $value:expr) => {{
        __check_size! {$from, $to}
        core::mem::ManuallyDrop::into_inner(
            crate::__priv_utils::Transmuter::<$from, $to> {
                from: core::mem::ManuallyDrop::new($value),
            }
            .to,
        )
    }};
}

// same as __priv_transmute, but specialized to known-Copy types.
macro_rules! __priv_transmute_from_copy {
    ($from:ty, $to:ty, $value:expr) => {{
        __check_size! {$from, $to}
        core::mem::ManuallyDrop::into_inner(
            crate::__priv_utils::TransmuterFromCopy::<$from, $to> { from: $value }.to,
        )
    }};
}

// Defined this to transmute references.
//
// Using this instead of `core::mem::transmute` to allow changing
// reference transmutes to transmute references to `?Sized` types
// (`std::mem::transmute` errors with those due to unknown sizes).
//
// this is unsafe to use for the same reason that `transmute::<&$from, &$to>` is,
// the types might not be compatible.
macro_rules! __priv_transmute_ref {
    ($from:ty, $to:ty, $reference:expr) => {{
        __check_size! {$from, $to}
        &*($reference as *const $from as *const $to)
    }};
}

// Defined this to transmute slices
//
// Using this instead of `core::mem::transmute`,
// because the data pointer and length are not guaranteed to be laid out in
// the same order for different `&[T]`s and `&[U]`s.
//
// this is unsafe to use for the same reason that `transmute::<&$from, &$to>` is,
// the types might not be compatible.
macro_rules! __priv_transmute_slice {
    ($from:ty, $to:ty, $reference:expr) => {{
        __check_size! {$from, $to}
        &*($reference as *const [$from] as *const [$to])
    }};
}
