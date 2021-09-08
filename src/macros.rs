macro_rules! __priv_transmute_unchecked {
    ($from:ty, $to:ty, $value:expr) => {
        core::mem::ManuallyDrop::into_inner(
            crate::__priv_utils::Transmuter::<$from, $to> {
                from: core::mem::ManuallyDrop::new($value),
            }
            .to,
        )
    };
}
macro_rules! __priv_transmute_ref_unchecked {
    ($from:ty, $to:ty, $reference:expr) => {
        crate::__priv_utils::PtrToRef::<$to> {
            ptr: $reference as *const $from as *const $to,
        }
        .reff
    };
}
