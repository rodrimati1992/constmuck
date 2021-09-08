use core::marker::PhantomData;

use bytemuck::Zeroable;

use crate::TypeSize;

mod __ {
    use super::*;

    /// Encodes a `T: Zeroable` bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    pub struct ImplsZeroed<T> {
        _private: PhantomData<fn() -> T>,
    }

    impl<T> Copy for ImplsZeroed<T> {}

    impl<T> Clone for ImplsZeroed<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Zeroable> ImplsZeroed<T> {
        /// Constructs an `ImplsZeroed`
        ///
        /// You can also use the [`infer`] macro to construct `ImplsZeroed` arguments.
        pub const NEW: Self = Self {
            _private: PhantomData,
        };
    }
}
pub use __::ImplsZeroed;

impl<T: Zeroable> crate::Infer for ImplsZeroed<T> {
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
pub const fn zeroed<T, const SIZE: usize>(_bounds: TypeSize<ImplsZeroed<T>, T, SIZE>) -> T {
    unsafe { __priv_transmute_unchecked!([u8; SIZE], T, [0; SIZE]) }
}
