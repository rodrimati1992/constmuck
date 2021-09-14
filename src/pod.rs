use core::{marker::PhantomData, mem};

use bytemuck::{Pod, PodCastError};

use crate::{ImplsCopy, ImplsZeroable};

mod __ {
    use super::*;

    /// Encodes a `T:`[`Pod`] bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// # Example
    ///
    /// ```rust
    /// use constmuck::{ImplsPod, cast, cast_ref_alt, cast_slice_alt, infer};
    ///
    /// {
    ///     // transmuting `i16` to `u16`
    ///     const FOO1: u16 = cast(-1i16, (ImplsPod::NEW, ImplsPod::NEW));
    ///    
    ///     // The same as the above constant
    ///     const FOO2: u16 = cast(-1i16, infer!());
    ///    
    ///     assert_eq!(FOO1, u16::MAX);
    ///     assert_eq!(FOO2, u16::MAX);
    /// }
    ///
    /// {
    ///     // transmuting `&i8` to `&u8`
    ///     const REFF: &u8 = cast_ref_alt(&-2i8, infer!());
    ///    
    ///     assert_eq!(REFF, &254);
    /// }
    ///
    /// {
    ///     // transmuting `&[u8]` to `&[i8]`
    ///     const REFF: &[i8] = cast_slice_alt(&[0u8, 127, 128, 255], infer!());
    ///    
    ///     assert_eq!(REFF, &[0i8, 127, -128, -1]);
    /// }
    ///
    /// ```
    pub struct ImplsPod<T> {
        pub impls_copy: ImplsCopy<T>,
        pub impls_zeroable: ImplsZeroable<T>,
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
            impls_copy: ImplsCopy::NEW,
            impls_zeroable: ImplsZeroable::NEW,
            _private: PhantomData,
        };
    }

    impl<T> ImplsPod<T> {
        const __NEW_UNCHECKED__: Self = unsafe {
            Self {
                impls_copy: ImplsCopy::new_unchecked(),
                impls_zeroable: ImplsZeroable::new_unchecked(),
                _private: PhantomData,
            }
        };

        /// Constructs an `ImplsPod<T>` without checking that `T` implements [`Pod`].
        ///
        /// # Safety
        ///
        /// You must ensure that `T` follows the
        /// [safety requirements of `Pod`](bytemuck::Pod#safety)
        ///
        /// ```rust
        /// use constmuck::{ImplsPod, cast, infer};
        ///
        /// #[repr(transparent)]
        /// struct Foo([u8; 4]);
        ///
        /// unsafe{
        ///     let bounds = (ImplsPod::new_unchecked(), ImplsPod::new_unchecked());
        ///     assert_eq!(cast::<u32, Foo>(12345678, bounds).0, 12345678u32.to_ne_bytes());
        /// }
        /// ```
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self::__NEW_UNCHECKED__
        }
    }
}
pub use __::ImplsPod;

impl<T: Pod> crate::Infer for ImplsPod<T> {
    const INFER: Self = Self::NEW;
}

/// For casting `T` into `U`
///
/// Requires both `T` and `U` to implement [`Pod`].
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
/// ```
pub const fn cast<T, U>(from: T, _bounds: (ImplsPod<T>, ImplsPod<U>)) -> U {
    unsafe {
        if mem::size_of::<T>() != mem::size_of::<U>() {
            let x = mem::size_of::<T>();
            let _: () = [/* the size of T and U is not the same */][x];
        }

        __priv_transmute!(T, U, from)
    }
}

/// For casting `T` into `U`
///
/// Requires both `T` and `U` to implement [`Pod`].
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
            Ok(__priv_transmute!(T, U, from))
        } else {
            // Pod requires types to be Copy, so this never causes a leak
            mem::forget(from);
            Err(bytemuck::PodCastError::SizeMismatch)
        }
    }
}

/// Cast a `&T` to `&U`
///
/// Requires both `T` and `U` to implement [`Pod`].
///
/// # Panics
///
/// This function panics in these cases:
/// - The alignment of `T` is larger than `U`
/// - The size of `T` is not equal to `U`
///
/// # Difference with `bytemuck`
///
/// This function requires `T` to have an alignment larger or equal to `U`,
/// while [`bytemuck::cast_ref`] only requires `from` to happen to be aligned
/// to `U`.
///
/// # Example
///
/// ```
/// use constmuck::{cast_ref_alt, infer};
///
/// const U8: &[u8; 2] = cast_ref_alt(&100u16.to_le(), infer!());
///
/// assert_eq!(U8[0], 100u8);
/// assert_eq!(U8[1], 0);
///
/// ```
pub const fn cast_ref_alt<T, U>(from: &T, bounds: (ImplsPod<T>, ImplsPod<U>)) -> &U {
    match try_cast_ref_alt(from, bounds) {
        Ok(x) => x,
        Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned) => {
            let x = mem::size_of::<T>();
            [/* the alignment of T is larger than U */][x]
        }
        Err(PodCastError::SizeMismatch | _) => {
            let x = mem::size_of::<T>();
            [/* the size of T and U is not the same */][x]
        }
    }
}

/// Cast a `&T` to `&U`
///
/// Requires both `T` and `U` to implement [`Pod`].
///
/// # Errors
///
/// This function returns errors in these cases:
/// - The alignment of `T` is larger than `U`, returning a
/// `Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)`.
///
/// - The size of `T` is not equal to `U`, returning a
/// `Err(PodCastError::SizeMismatch)`.
///
/// # Difference with `bytemuck`
///
/// This function requires `T` to have an alignment larger or equal to `U`,
/// while [`bytemuck::try_cast_ref`] only requires `from` to happen to be aligned
/// to `U`.
///
/// # Example
///
/// ```
/// use constmuck::PodCastError;
/// use constmuck::{infer, try_cast_ref_alt};
///
/// const U8: Result<&[u8; 2], PodCastError> = try_cast_ref_alt(&100u16.to_le(), infer!());
/// const ERR_SIZE : Result<&u8, PodCastError> = try_cast_ref_alt(&100u16.to_le(), infer!());
/// const ERR_ALIGN: Result<&u16, PodCastError> = try_cast_ref_alt(&100u8, infer!());
///
/// assert_eq!(U8, Ok(&[100u8, 0]));
/// assert_eq!(ERR_SIZE, Err(PodCastError::SizeMismatch));
/// assert_eq!(ERR_ALIGN, Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned));
///
/// ```
pub const fn try_cast_ref_alt<T, U>(
    from: &T,
    _bounds: (ImplsPod<T>, ImplsPod<U>),
) -> Result<&U, crate::PodCastError> {
    unsafe {
        if mem::align_of::<T>() < mem::align_of::<U>() {
            Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)
        } else if mem::size_of::<T>() != mem::size_of::<U>() {
            Err(PodCastError::SizeMismatch)
        } else {
            Ok(__priv_transmute_ref!(T, U, from))
        }
    }
}
