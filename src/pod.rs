use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem,
};

use bytemuck::{Pod, PodCastError};

use crate::{IsCopy, IsZeroable};

/// Constructs an [`IsPod<$T>`](struct@crate::IsPod),
/// requires `$T:`[`Pod`](trait@Pod).
///
/// This has an optional type argument (`$T`) that defaults to
/// infering the type if not passed.
///
/// # Example
///
/// ```rust
/// use constmuck::{IsPod, cast, infer};
///
/// // transmuting `i16` to `u16`
/// const FOO: u16 = cast(-1i16, (IsPod!(), IsPod!()));
/// const BAR: u16 = cast(-1, (IsPod!(i16), IsPod!(u16)));
/// // A more concise way to call `constmuck::cast` when the types are inferred.
/// const BAZ: u16 = cast(-1i16, infer!());
///
/// assert_eq!(FOO, u16::MAX);
/// assert_eq!(BAR, u16::MAX);
/// assert_eq!(BAZ, u16::MAX);
///
/// ```
#[macro_export]
macro_rules! IsPod {
    () => {
        <$crate::IsPod<_> as $crate::Infer>::INFER
    };
    ($T:ty) => {
        <$crate::IsPod<$T> as $crate::Infer>::INFER
    };
}

mod __ {
    use super::*;

    /// Encodes a `T:`[`Pod`](trait@Pod) bound as a value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use constmuck::{IsPod, cast, cast_ref_alt, cast_slice_alt, infer};
    ///
    /// {
    ///     // transmuting `i16` to `u16`
    ///     // The four lines below are equivalent
    ///     const FOO1: u16 = cast(-1i16, (IsPod::NEW, IsPod::NEW));
    ///     const FOO2: u16 = cast(-1i16, (IsPod!(), IsPod!()));
    ///     const FOO3: u16 = cast(-1, (IsPod!(i16), IsPod!(u16)));
    ///     const FOO4: u16 = cast(-1i16, infer!());
    ///    
    ///     assert_eq!(FOO1, u16::MAX);
    ///     assert_eq!(FOO2, u16::MAX);
    ///     assert_eq!(FOO3, u16::MAX);
    ///     assert_eq!(FOO4, u16::MAX);
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
    pub struct IsPod<T> {
        /// All types that are [`Pod`](trait@Pod) are [`Copy`]
        pub is_copy: IsCopy<T>,
        /// All types that are [`Pod`](trait@Pod) are [`Zeroable`](trait@bytemuck::Zeroable)
        pub is_zeroable: IsZeroable<T>,
        // The lifetime of `T` is invariant,
        // just in case that it's unsound for lifetimes to be co/contravariant.
        _private: PhantomData<fn(T) -> T>,
    }

    impl<T> Debug for IsPod<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("IsPod")
        }
    }

    impl<T> Copy for IsPod<T> {}

    impl<T> Clone for IsPod<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Pod> IsPod<T> {
        /// Constructs an `IsPod`.
        ///
        /// You can also use the [`IsPod`](macro@crate::IsPod) or [`ìnfer`](macro@crate::infer)
        /// macros to construct `IsPod` arguments.
        pub const NEW: Self = Self {
            is_copy: IsCopy::NEW,
            is_zeroable: IsZeroable::NEW,
            _private: PhantomData,
        };
    }

    impl<T> IsPod<T> {
        const __NEW_UNCHECKED__: Self = unsafe {
            Self {
                is_copy: IsCopy::new_unchecked(),
                is_zeroable: IsZeroable::new_unchecked(),
                _private: PhantomData,
            }
        };

        /// Constructs an `IsPod<T>` without checking that `T` implements [`Pod`](trait@Pod).
        ///
        /// # Safety
        ///
        /// You must ensure that `T` follows the
        /// [safety requirements of `Pod`](bytemuck::Pod#safety)
        ///
        /// ```rust
        /// use constmuck::{IsPod, cast};
        ///
        /// #[repr(transparent)]
        /// struct Foo([u8; 4]);
        ///
        /// unsafe{
        ///     let bounds = (IsPod::new_unchecked(), IsPod::new_unchecked());
        ///     assert_eq!(cast::<u32, Foo>(12345678, bounds).0, 12345678u32.to_ne_bytes());
        /// }
        /// ```
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self::__NEW_UNCHECKED__
        }
    }
}
pub use __::IsPod;

impl<T: Pod> crate::Infer for IsPod<T> {
    const INFER: Self = Self::NEW;
}

/// Casts `T` into `U`
///
/// Requires both `T` and `U` to implement [`Pod`](trait@Pod).
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
pub const fn cast<T, U>(from: T, _bounds: (IsPod<T>, IsPod<U>)) -> U {
    unsafe {
        if mem::size_of::<T>() != mem::size_of::<U>() {
            let x = mem::size_of::<T>();
            let _: () = [/* the size of T and U is not the same */][x];
        }

        __priv_transmute!(T, U, from)
    }
}

/// Tries to cast `T` into `U`
///
/// Requires both `T` and `U` to implement [`Pod`](trait@Pod).
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
    _bounds: (IsPod<T>, IsPod<U>),
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
/// Requires both `T` and `U` to implement [`Pod`](trait@Pod).
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
/// while [`bytemuck::cast_ref`] only requires the `from` reference
/// to happen to be aligned to `U`.
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
pub const fn cast_ref_alt<T, U>(from: &T, bounds: (IsPod<T>, IsPod<U>)) -> &U {
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

/// Tries to cast `&T` to `&U`
///
/// Requires both `T` and `U` to implement [`Pod`](trait@Pod).
///
/// # Errors
///
/// This function returns errors in these cases:
/// - The alignment of `T` is larger than `U`, returning a
/// `Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)`.
/// <br>(using this instead of `PodCastError::AlignmentMismatch` because that
/// is not returned by [`bytemuck::try_cast_ref`])
///
/// - The size of `T` is not equal to `U`, returning a
/// `Err(PodCastError::SizeMismatch)`.
///
/// # Difference with `bytemuck`
///
/// This function requires `T` to have an alignment larger or equal to `U`,
/// while [`bytemuck::try_cast_ref`] only requires the `from` reference
/// to happen to be aligned to `U`.
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
    _bounds: (IsPod<T>, IsPod<U>),
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
