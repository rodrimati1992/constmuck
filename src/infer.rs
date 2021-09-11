/// For constructing types that implement [`Infer`], this includes `Impls*` types.
///
/// # Example
///
/// ```rust
/// use constmuck::{cast, infer};
/// use constmuck::{ImplsPod, ImplsTransparentWrapper};
///
/// use std::num::Wrapping;
///
/// const ARR: [u8; 4] = cast([-3i8, -2, -1, 0], infer!());
/// assert_eq!(ARR, [253, 254, 255, 0]);
///
/// const fn requires_pod<T>(_bounds: ImplsPod<T>) {}
/// requires_pod::<u32>(infer!());
/// // the same as the above call
/// requires_pod(infer!(ImplsPod<u32>));
///
/// const fn requires_2_bounds<T, U>(_bounds: (ImplsPod<T>, ImplsTransparentWrapper<U, T>)) {}
/// requires_2_bounds::<u32, Wrapping<u32>>(infer!());
/// // the same as the above call
/// requires_2_bounds(infer!((ImplsPod<u32>, ImplsTransparentWrapper<Wrapping<u32>, u32>)));
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

/// For constructing `Impls*` types (values that represent trait bounds),
/// and tuples of them.
///
/// For a more concise way to write to `Infer::INFER`, there's the [`infer`] macro.
///
/// # Example
///
/// ```rust
/// use constmuck::cast;
/// use constmuck::{Infer, ImplsPod, ImplsTransparentWrapper};
///
/// use std::num::Wrapping;
///
/// const ARR: [i8; 3] = cast([3u8, 5, u8::MAX - 1], Infer::INFER);
/// assert_eq!(ARR, [3, 5, -2]);
///
/// const fn requires_pod<T>(_bounds: ImplsPod<T>) {}
/// requires_pod::<u32>(Infer::INFER);
///
/// const fn requires_2_bounds<T, U>(_bounds: (ImplsPod<T>, ImplsTransparentWrapper<U, T>)) {}
/// requires_2_bounds::<u32, Wrapping<u32>>(Infer::INFER);
///  
/// ```
pub trait Infer: Sized + Copy {
    /// Constructs this type.
    const INFER: Self;
}

macro_rules! impl_tuple {
    ($($ty:ident),*) => {
        impl<$($ty),*> Infer for ($($ty,)*)
        where $($ty: Infer,)*
        {
            const INFER: Self = ($($ty::INFER,)*);
        }
    };
}

impl_tuple! {}
impl_tuple! {A}
impl_tuple! {A,B}
impl_tuple! {A,B,C}
impl_tuple! {A,B,C,D}
impl_tuple! {A,B,C,D,E}
impl_tuple! {A,B,C,D,E,F}
impl_tuple! {A,B,C,D,E,F,G}
impl_tuple! {A,B,C,D,E,F,G,H}
