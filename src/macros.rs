#[cfg(feature = "debug_checks")]
macro_rules! __check_size {
    ($from:ty, $to:ty) => (
        if core::mem::size_of::<$from>() != core::mem::size_of::<$to>() {
            let size_of_from = core::mem::size_of::<$from>();
            [(/* expected transmute not to change the size */)][size_of_from];
            loop{}
        }
    )
}

#[cfg(not(feature = "debug_checks"))]
macro_rules! __check_size {
    ($from:ty, $to:ty) => {};
}

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

macro_rules! __priv_transmute_from_copy {
    ($from:ty, $to:ty, $value:expr) => {{
        __check_size! {$from, $to}
        core::mem::ManuallyDrop::into_inner(
            crate::__priv_utils::TransmuterFromCopy::<$from, $to> { from: $value }.to,
        )
    }};
}

macro_rules! __priv_transmute_ref {
    ($from:ty, $to:ty, $reference:expr) => {{
        __check_size! {$from, $to}
        crate::__priv_utils::PtrToRef::<$to> {
            ptr: $reference as *const $from as *const $to,
        }
        .reff
    }};
}

macro_rules! __priv_transmute_slice {
    ($from:ty, $to:ty, $reference:expr) => {{
        __check_size! {$from, $to}
        crate::__priv_utils::PtrToRef::<[$to]> {
            ptr: $reference as *const [$from] as *const [$to],
        }
        .reff
    }};
}
