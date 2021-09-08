use core::{marker::PhantomData, mem};

use bytemuck::Pod;

mod __ {
    use super::*;

    /// Encodes a `T: Pod` bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    pub struct ImplsPod<T> {
        _private: PhantomData<fn() -> T>,
    }

    impl<T> Copy for ImplsPod<T> {}

    impl<T> Clone for ImplsPod<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Pod> ImplsPod<T> {
        /// Constructs an `ImplsPod`
        ///
        /// You can also use the [`infer`] macro to construct `ImplsPod` arguments.
        pub const NEW: Self = Self {
            _private: PhantomData,
        };
    }
}
pub use __::ImplsPod;

impl<T: Pod> crate::Infer for ImplsPod<T> {
    const INFER: Self = Self::NEW;
}

/// For casting `T` into `U`
///
/// # Panics
///
/// This panics if `T` is not the same size as `U`
///
/// # Example
///
/// ```rust
/// use constmuck::{cast, infer};
///
/// const LE_BYTES: [u8; 4] = cast(0xAB1E_BEEF_u32.to_le(), infer!());
///
/// assert_eq!(LE_BYTES, 0xAB1E_BEEF_u32.to_le_bytes());
///
///
/// ```
pub const fn cast<T, U>(from: T, _bounds: (ImplsPod<T>, ImplsPod<U>)) -> U {
    unsafe {
        let same_size = mem::size_of::<T>() == mem::size_of::<U>();
        [(/* expected T and U of the same size */)][(!same_size) as usize];

        __priv_transmute_unchecked!(T, U, from)
    }
}

/// For casting `T` into `U`
///
/// # Errors
///
/// This returns an `Err(PodCastError::SizeMismatch)` when `T` isn't the same size as `U`.
///
/// # Example
///
/// ```rust
/// use constmuck::PodCastError;
/// use constmuck::{try_cast, infer};
///
/// const OK: Result<i32, PodCastError> = try_cast(u32::MAX, infer!());
/// const ERR: Result<[u8; 4], PodCastError> = try_cast(100_u16, infer!());
///
/// assert_eq!(OK, Ok(-1));
/// assert_eq!(ERR, Err(PodCastError::SizeMismatch));
///
///
/// ```
pub const fn try_cast<T, U>(
    from: T,
    _bounds: (ImplsPod<T>, ImplsPod<U>),
) -> Result<U, crate::PodCastError> {
    unsafe {
        if mem::size_of::<T>() == mem::size_of::<U>() {
            Ok(__priv_transmute_unchecked!(T, U, from))
        } else {
            // Pod requires types to be Copy, so this never causes a leak
            mem::forget(from);
            Err(bytemuck::PodCastError::SizeMismatch)
        }
    }
}
