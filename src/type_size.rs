use core::{
    marker::PhantomData,
    mem::{self, ManuallyDrop},
};

use crate::Infer;

/// Constructs a [`TypeSize`]
///
/// # Example
///
/// Making a `oned` function
///
/// ```rust
/// use constmuck::{ImplsPod, TypeSize};
/// use constmuck::{infer, type_size};
///
/// pub const fn oned<T, const SIZE: usize>(bound: TypeSize<ImplsPod<T>, T, SIZE>) -> T {
///     constmuck::cast::<[u8; SIZE], T>(
///         [1; SIZE],
///         // `infer!()` here constructs an `ImplsPod<[u8; SIZE]>`
///         //
///         // `bound.into_bounds()` here returns an `ImplsPod<T>`,
///         // the first type argument of `bounds`
///         (infer!(), bound.into_bounds())
///     )
/// }
///
/// const U64: u64 = oned(type_size!(u64));
/// const ONES: [u8; 5] = oned(type_size!([u8; 5]));
///
/// assert_eq!(U64, 0x01_01_01_01_01_01_01_01);
/// assert_eq!(ONES, [1, 1, 1, 1, 1]);
///
/// ```
#[macro_export]
macro_rules! type_size {
    ($ty:ty) => {
        $crate::TypeSize::<_, $ty, { $crate::__::size_of::<$ty>() }>::__NEW__
    };
}

/// For passing a type along with its size, constructible with the [`type_size`] macro.
///
/// The `B` type parameter can be any type that implements [`Infer`],
/// and is implicitly constructed by the [`type_size`] macro.
///
/// # Example
///
/// Making a `max_bit_pattern` function
///
/// ```rust
/// use constmuck::{ImplsPod, TypeSize};
/// use constmuck::{infer, type_size};
///
/// pub const fn max_bit_pattern<T, const SIZE: usize>(bound: TypeSize<ImplsPod<T>, T, SIZE>) -> T {
///     constmuck::cast::<[u8; SIZE], T>(
///         [u8::MAX; SIZE],
///         // `infer!()` here constructs an `ImplsPod<[u8; SIZE]>`
///         //
///         // `bound.into_bounds()` here returns an `ImplsPod<T>`,
///         // the first type argument of `bounds`
///         (infer!(), bound.into_bounds())
///     )
/// }
///
/// const U64: u64 = max_bit_pattern(type_size!(u64));
/// const U8S: [u8; 5] = max_bit_pattern(type_size!([u8; 5]));
/// const I8S: [i8; 5] = max_bit_pattern(type_size!([i8; 5]));
///
/// assert_eq!(U64, u64::MAX);
/// assert_eq!(U8S, [u8::MAX; 5]);
/// assert_eq!(I8S, [-1i8; 5]);
///
/// ```
pub struct TypeSize<B, T, const SIZE: usize> {
    bounds: ManuallyDrop<B>,
    _private: PhantomData<T>,
}

impl<B: Copy, T, const SIZE: usize> Copy for TypeSize<B, T, SIZE> {}

impl<B: Copy, T, const SIZE: usize> Clone for TypeSize<B, T, SIZE> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Infer, T, const SIZE: usize> TypeSize<B, T, SIZE> {
    #[doc(hidden)]
    pub const __NEW__: Self = {
        if mem::size_of::<T>() != SIZE {
            [/* WTF */][mem::size_of::<T>()]
        } else {
            Self {
                bounds: ManuallyDrop::new(Infer::INFER),
                _private: PhantomData,
            }
        }
    };
}

impl<B, T, const SIZE: usize> TypeSize<B, T, SIZE> {
    pub const fn into_bounds(self) -> B {
        ManuallyDrop::into_inner(self.bounds)
    }
}
