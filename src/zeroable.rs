use core::marker::PhantomData;

use bytemuck::Zeroable;

use crate::TypeSize;

/// Constructs an [`IsZeroable<$T>`](struct@crate::IsZeroable),
/// requires [`$T: Zeroable`](trait@bytemuck::Zeroable).
///
/// This has an optional type argument (`$T`) that default to
/// infering the type if not passed.
///
/// This macro is defined for completeness' sake,
/// no function in this crate takes `IsZeroable` by itself,
/// always a [`TypeSize<T, IsZeroable<T>, _>`](struct@crate::TypeSize),
/// which can be constructed with the
/// [`TypeSize`](macro@crate::TypeSize) macro.
///
/// Related: the [`copying`](crate::copying) module
///
/// # Example
///
/// ```rust
/// use constmuck::{IsZeroable, TypeSize};
///
/// const FOO: IsZeroable<u32> = IsZeroable!();
/// assert_eq!(constmuck::zeroed(TypeSize!(u32).with_bounds(FOO)), 0u32);
/// // alternatively, the typical way to call `constmuck::zeroed`.
/// assert_eq!(constmuck::zeroed(TypeSize!(u32)), 0u32);
///
///
/// const BAR: IsZeroable<u8> = IsZeroable!(u8);
/// assert_eq!(constmuck::zeroed_array(TypeSize!(u8).with_bounds(BAR)), [0u8; 4]);
/// // alternatively, the typical way to call `constmuck::zeroed_array`.
/// assert_eq!(constmuck::zeroed_array(TypeSize!(u8)), [0u8; 4]);
///
///
/// ```
#[macro_export]
macro_rules! IsZeroable {
    () => {
        <$crate::IsZeroable<_> as $crate::Infer>::INFER
    };
    ($T:ty) => {
        <$crate::IsZeroable<$T> as $crate::Infer>::INFER
    };
}

mod __ {
    use super::*;

    /// Encodes a `T:`[`Zeroable`] bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// Related: the [`zeroed`] and [`zeroed_array`] functions.
    pub struct IsZeroable<T> {
        _private: PhantomData<fn() -> T>,
    }

    impl<T> Copy for IsZeroable<T> {}

    impl<T> Clone for IsZeroable<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Zeroable> IsZeroable<T> {
        /// Constructs an `IsZeroable`
        ///
        /// You can also use the [`IsZeroable`](macro@crate::IsZeroable)
        /// macro to construct `IsZeroable` arguments.
        pub const NEW: Self = Self {
            _private: PhantomData,
        };
    }

    impl<T> IsZeroable<T> {
        const __NEW_UNCHECKED__: Self = Self {
            _private: PhantomData,
        };

        /// Constructs an `IsZeroable<T>` without checking that `T` implements [`Zeroable`].
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
pub use __::IsZeroable;

impl<T: Zeroable> crate::Infer for IsZeroable<T> {
    const INFER: Self = Self::NEW;
}

/// For safely getting a [`std::mem::zeroed`](core::mem::zeroed) `T`.
///
/// This function requires that `T` implements [`Zeroable`](bytemuck::Zeroable).
///
/// # Example
///
/// ```rust
/// use constmuck::{TypeSize, zeroed};
///
/// const BYTES: [u8; 4] = zeroed(TypeSize!([u8; 4]));
/// const CHARS: [char; 4] = zeroed(TypeSize!([char; 4]));
///
/// assert_eq!(BYTES, [0, 0, 0, 0]);
/// assert_eq!(CHARS, ['\0', '\0', '\0', '\0']);
///
///
/// ```
pub const fn zeroed<T, const SIZE: usize>(_bounds: TypeSize<T, IsZeroable<T>, SIZE>) -> T {
    unsafe { __priv_transmute!([u8; SIZE], T, [0; SIZE]) }
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
/// use constmuck::{TypeSize, zeroed_array};
///
/// const BYTES: [u8; 2] = zeroed_array(TypeSize!(u8));
/// assert_eq!(BYTES, [0, 0]);
/// // using `TypeSize::zeroed_array` to pass the length of the returned array.
/// assert_eq!(TypeSize!(u8).zeroed_array::<2>(), [0, 0]);
///
///
/// const CHARS: [char; 4] = zeroed_array(TypeSize!(char));
/// assert_eq!(CHARS, ['\0', '\0', '\0', '\0']);
/// // using `TypeSize::zeroed_array` to pass the length of the returned array.
/// assert_eq!(TypeSize!(char).zeroed_array::<4>(), ['\0', '\0', '\0', '\0']);
///
///
///
///
///
/// ```
pub const fn zeroed_array<T, const SIZE: usize, const LEN: usize>(
    _bounds: TypeSize<T, IsZeroable<T>, SIZE>,
) -> [T; LEN] {
    unsafe { __priv_transmute!([[u8; SIZE]; LEN], [T; LEN], [[0u8; SIZE]; LEN]) }
}
