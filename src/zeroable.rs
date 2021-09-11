use core::marker::PhantomData;

use bytemuck::Zeroable;

use crate::TypeSize;

mod __ {
    use super::*;

    /// Encodes a `T:`[`Zeroable`] bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// Related: the [`zeroed`] and [`zeroed_array`] functions.
    pub struct ImplsZeroable<T> {
        _private: PhantomData<fn() -> T>,
    }

    impl<T> Copy for ImplsZeroable<T> {}

    impl<T> Clone for ImplsZeroable<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Zeroable> ImplsZeroable<T> {
        /// Constructs an `ImplsZeroable`
        ///
        /// You can also use the [`infer`] macro to construct `ImplsZeroable` arguments.
        pub const NEW: Self = Self {
            _private: PhantomData,
        };
    }

    impl<T> ImplsZeroable<T> {
        const __NEW_UNCHECKED__: Self = Self {
            _private: PhantomData,
        };

        /// Constructs an `ImplsZeroable<T>` without checking that `T` implements [`Zeroable`].
        ///
        /// # Safety
        ///
        /// You must ensure that `T` follows the
        /// [safety requirements of `Zeroable`](bytemuck::Zeroable#safety)
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self::__NEW_UNCHECKED__
        }
    }
}
pub use __::ImplsZeroable;

impl<T: Zeroable> crate::Infer for ImplsZeroable<T> {
    const INFER: Self = Self::NEW;
}

/// For safely getting a [`std::mem::zeroed`](core::mem::zeroed) `T`.
///
/// This function requires that `T` implements [`Zeroable`](bytemuck::Zeroable).
///
/// # Example
///
/// ```rust
/// use constmuck::{zeroed, type_size};
///
/// const BYTES: [u8; 4] = zeroed(type_size!([u8; 4]));
/// const CHARS: [char; 4] = zeroed(type_size!([char; 4]));
///
/// assert_eq!(BYTES, [0, 0, 0, 0]);
/// assert_eq!(CHARS, ['\0', '\0', '\0', '\0']);
///
///
/// ```
pub const fn zeroed<T, const SIZE: usize>(_bounds: TypeSize<ImplsZeroable<T>, T, SIZE>) -> T {
    unsafe { __priv_transmute_unchecked!([u8; SIZE], T, [0; SIZE]) }
}

/// For safely getting a [`std::mem::zeroed`](core::mem::zeroed) `[T; N]`.
///
/// This function requires that `T` implements [`Zeroable`](bytemuck::Zeroable).
///
/// To specify the length of the returned array, [`TypeSize::zeroed_array`]
/// can be used instead.
///
/// # Example
///
/// ```rust
/// use constmuck::{zeroed_array, type_size};
///
/// const BYTES: [u8; 2] = zeroed_array(type_size!(u8));
/// const CHARS: [char; 4] = zeroed_array(type_size!(char));
///
/// assert_eq!(BYTES, [0, 0]);
///
/// // you can use `TypeSize::zeroed_array` like here to pass the length of the returned array.
/// assert_eq!(type_size!(u8).zeroed_array::<2>(), [0, 0]);
///
///
/// assert_eq!(CHARS, ['\0', '\0', '\0', '\0']);
/// assert_eq!(type_size!(char).zeroed_array::<4>(), ['\0', '\0', '\0', '\0']);
///
///
///
///
///
/// ```
pub const fn zeroed_array<T, const SIZE: usize, const LEN: usize>(
    _bounds: TypeSize<ImplsZeroable<T>, T, SIZE>,
) -> [T; LEN] {
    unsafe { __priv_transmute_unchecked!([[u8; SIZE]; LEN], [T; LEN], [[0u8; SIZE]; LEN]) }
}
