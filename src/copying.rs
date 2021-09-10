//! For copying values in generic contexts.
//!
//! Related: [`ImplsCopy`] type, [`type_size`] macro.

use core::{marker::PhantomData, mem::MaybeUninit};

use crate::TypeSize;

pub(crate) mod impls_copy {
    use super::*;

    /// Encodes a `T: Copy` bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// Related: the [`copying`](crate::copying) module
    pub struct ImplsCopy<T> {
        _private: PhantomData<fn() -> T>,
    }

    impl<T> Copy for ImplsCopy<T> {}

    impl<T> Clone for ImplsCopy<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Copy> ImplsCopy<T> {
        /// Constructs an `ImplsCopy`
        ///
        /// You can also use the [`infer`] macro to construct `ImplsCopy` arguments.
        pub const NEW: Self = Self {
            _private: PhantomData,
        };
    }
}
use impls_copy::ImplsCopy;

impl<T: Copy> crate::Infer for ImplsCopy<T> {
    const INFER: Self = Self::NEW;
}

/// Copies a `T` from a `&T`
///
/// Requires that `T` implements `Copy`
///
/// # Example
///
/// Making a `pair` function.
///
/// ```rust
/// use constmuck::{copying, type_size};
/// use constmuck::{ImplsCopy, TypeSize};
///
/// const fn pair<T, const SIZE: usize>(
///     reff: &T,
///     bounds: TypeSize<ImplsCopy<T>, T, SIZE>
/// ) -> [T; 2] {
///     [copying::copy(reff, bounds), copying::copy(reff, bounds)]
/// }
///
/// const PAIR_U8: [u8; 2] = pair(&128, type_size!(u8));
/// const PAIR_STR: [&str; 2] = pair(&"hello", type_size!(&str));
///
/// assert_eq!(PAIR_U8, [128, 128]);
///
/// assert_eq!(PAIR_STR, ["hello", "hello"]);
///
/// ```
pub const fn copy<T, const SIZE: usize>(reff: &T, bounds: TypeSize<ImplsCopy<T>, T, SIZE>) -> T {
    unsafe {
        __priv_transmute_from_copy_unchecked!(
            MaybeUninit<[u8; SIZE]>,
            T,
            *crate::slice_fns::maybe_uninit_bytes_of(reff, bounds)
        )
    }
}

/// Creates a `[T; ARR_LEN]` by copying from a `&T`
///
/// Requires that `T` implements `Copy`
///
/// # Example
///
/// ```rust
/// use constmuck::{copying, type_size};
///
/// const PAIR: [Option<u8>; 2] = copying::repeat(&None, type_size!(Option<u8>));
/// const TEN: [&str; 10] = copying::repeat(&"world", type_size!(&str));
///
/// assert_eq!(PAIR, [None, None]);
///
/// assert_eq!(TEN, ["world"; 10]);
///
/// ```
pub const fn repeat<T, const SIZE: usize, const ARR_LEN: usize>(
    reff: &T,
    bounds: TypeSize<ImplsCopy<T>, T, SIZE>,
) -> [T; ARR_LEN] {
    unsafe {
        __priv_transmute_from_copy_unchecked!(
            [MaybeUninit<[u8; SIZE]>; ARR_LEN],
            [T; ARR_LEN],
            [*crate::slice_fns::maybe_uninit_bytes_of(reff, bounds); ARR_LEN]
        )
    }
}
