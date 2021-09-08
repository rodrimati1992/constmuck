/// For constructing `Impls*` types (values that represent trait bounds).
///
/// # Example
///
/// ```rust
/// use constmuck::{cast, infer};
/// use constmuck::ImplsPod;
///
/// const ARR: [u8; 4] = cast([-3i8, -2, -1, 0], infer!());
/// assert_eq!(ARR, [253, 254, 255, 0]);
///
/// const fn requires_pod<T>(_bounds: ImplsPod<T>) {}
/// requires_pod::<u32>(infer!());
///
/// const fn requires_2_pods<T, U>(_bounds: (ImplsPod<T>, ImplsPod<U>)) {}
/// requires_2_pods::<u32, u64>(infer!());
///  
/// ```
#[macro_export]
macro_rules! infer {
    () => {
        $crate::Infer::INFER
    };
    ($ty:ty) => {
        <$ty as $crate::Infer>::INFER
    };
}

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
