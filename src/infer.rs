/// For constructing types that implement [`Infer`], this includes `Is*` types.
///
/// This is best used when the constructed type can be inferred.
///
/// The type argument (`$ty`) is optional, inferred when not passed.
///
/// # Alternatives
///
/// `constmuck` types have their own macros for constructing them,
/// which are more concise with explicit type arguments,
/// and help type inference.
///
/// These are the macros:
///
/// - [`IsContiguous`](macro@crate::IsContiguous):
/// Constructs an [`IsContiguous`](struct@crate::IsContiguous) type.
///
/// - [`IsCopy`](macro@crate::IsCopy):
/// Constructs an [`IsCopy`](struct@crate::IsCopy) type.
///
/// - [`IsPod`](macro@crate::IsPod):
/// Constructs an [`IsPod`](struct@crate::IsPod) type.
///
/// - [`IsTW`](macro@crate::IsTW):
/// Constructs an [`IsTransparentWrapper`](struct@crate::IsTransparentWrapper) type.
///
/// - [`IsZeroable`](macro@crate::IsZeroable):
/// Constructs an [`IsZeroable`](struct@crate::IsZeroable) type.
///
/// - [`TypeSize`](macro@crate::TypeSize):
/// Constructs an [`TypeSize`](struct@crate::TypeSize) type.
///
///
/// # Example
///
/// ### Basic
///
/// ```rust
/// use constmuck::{infer, wrapper};
///
/// use std::num::Wrapping as W;
///
/// const FOO: &[W<u8>] = wrapper::wrap_slice(&[3, 2, 1, 0], infer!());
/// assert_eq!(FOO, [W(3), W(2), W(1), W(0)]);
///
/// let bar = wrapper::peel_slice::<W<u8>, u8>(FOO, infer!());
/// assert_eq!(bar, [3, 2, 1, 0]);
///
///
/// ```
///
/// ### Tuple
///
/// `infer` can contruct tuples of types that implement [`Infer`].
///
/// ```rust
/// use constmuck::infer;
/// use constmuck::{IsPod, IsTW, IsTransparentWrapper};
///
/// use std::num::Wrapping;
///
/// const fn requires_2_bounds<T, U>(_bounds: (IsPod<T>, IsTransparentWrapper<U, T>)) {}
/// requires_2_bounds::<u32, Wrapping<u32>>(infer!());
///
/// // the same as the above call
/// requires_2_bounds(infer!((IsPod<u32>, IsTransparentWrapper<Wrapping<u32>, u32>)));
///
/// // using more specific macros
/// requires_2_bounds((IsPod!(u32), IsTW!(Wrapping<u32>, u32)));
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

/// For constructing `Is*` types (values that represent trait bounds),
/// and tuples of them.
///
/// For a more concise way to write to `Infer::INFER`, there's the [`infer`] macro.
///
/// # Example
///
/// ```rust
/// use constmuck::cast;
/// use constmuck::{Infer, IsPod, IsTransparentWrapper};
///
/// use std::num::Wrapping;
///
/// const ARR: [i8; 3] = cast([3u8, 5, u8::MAX - 1], Infer::INFER);
/// assert_eq!(ARR, [3, 5, -2]);
///
/// const fn requires_pod<T>(_bounds: IsPod<T>) {}
/// requires_pod::<u32>(Infer::INFER);
///
/// const fn requires_2_bounds<T, U>(_bounds: (IsPod<T>, IsTransparentWrapper<U, T>)) {}
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
