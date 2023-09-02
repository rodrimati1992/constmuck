use core::mem;

use bytemuck::{AnyBitPattern, NoUninit, PodCastError};

use crate::__priv_utils::Packed;

/// Casts `T` into `U`
///
/// # Panics
///
/// This panics if `T` is not the same size as `U`
///
/// # Example
///
/// ```rust
/// use constmuck::cast;
///
/// const LE_BYTES: [u8; 4] = cast(0xAB1E_BEEF_u32.to_le());
///
/// assert_eq!(LE_BYTES, 0xAB1E_BEEF_u32.to_le_bytes());
///
/// ```
#[track_caller]
pub const fn cast<T, U>(from: T) -> U
where
    T: NoUninit,
    U: AnyBitPattern,
{
    unsafe {
        if mem::size_of::<T>() != mem::size_of::<U>() {
            crate::__priv_utils::unequal_size_panic(mem::size_of::<T>(), mem::size_of::<U>())
        }

        // safety: the `_bounds` parameter guarantees that both `T` and `U`
        // contain no padding and are valid for all bitpatterns.
        // They are both guaranteed to be the same size by the above conditional.
        __priv_transmute!(T, U, from)
    }
}

/// Tries to cast `T` into `U`
///
/// # Errors
///
/// This returns an `Err(PodCastError::SizeMismatch)` when `T` isn't the same size as `U`.
///
/// # Example
///
/// ```rust
/// use constmuck::PodCastError;
/// use constmuck::try_cast;
///
/// const OK: Result<i32, PodCastError> = try_cast(u32::MAX);
/// const ERR: Result<[u8; 4], PodCastError> = try_cast(100_u16);
///
/// assert_eq!(OK, Ok(-1));
/// assert_eq!(ERR, Err(PodCastError::SizeMismatch));
///
///
/// ```
pub const fn try_cast<T, U>(from: T) -> Result<U, PodCastError>
where
    T: NoUninit,
    U: AnyBitPattern,
{
    unsafe {
        if mem::size_of::<T>() == mem::size_of::<U>() {
            // safety: the `_bounds` parameter guarantees that both `T` and `U`
            // contain no padding and are valid for all bitpatterns.
            // They are both guaranteed the same size in this branch.
            Ok(__priv_transmute!(T, U, from))
        } else {
            Err(PodCastError::SizeMismatch)
        }
    }
}

/// Cast a `&T` to `&U`
///
/// # Panics
///
/// This function panics in these cases:
/// - The alignment of `T` is larger than `U`
/// - The size of `T` is not equal to `U`
///
/// # Difference with `bytemuck`
///
/// This function requires `T` to have an alignment larger than or equal to `U`.
/// [`bytemuck::cast_ref`] allows `T` to have  to have a lower alignment than `U`,
/// so long as the `from` reference happens to be aligned to `U`.
///
/// # Example
///
/// ```
/// use constmuck::cast_ref_alt;
///
/// const U8: &[u8; 2] = cast_ref_alt(&100u16.to_le());
///
/// assert_eq!(U8[0], 100u8);
/// assert_eq!(U8[1], 0);
///
/// ```
#[track_caller]
pub const fn cast_ref_alt<T, U>(from: &T) -> &U
where
    T: NoUninit,
    U: AnyBitPattern,
{
    match try_cast_ref_alt(from) {
        Ok(x) => x,
        Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned) => {
            crate::__priv_utils::incompatible_alignment_panic(
                mem::align_of::<T>(),
                mem::align_of::<U>(),
            )
        }
        Err(PodCastError::SizeMismatch | _) => {
            crate::__priv_utils::unequal_size_panic(mem::size_of::<T>(), mem::size_of::<U>())
        }
    }
}

/// Tries to cast `&T` to `&U`
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
/// This function requires `T` to have an alignment larger than or equal to `U`.
/// [`bytemuck::try_cast_ref`] allows `T` to have a lower alignment than `U`,
/// so long as the `from` reference happens to be aligned to `U`.
///
/// # Example
///
/// ```
/// use constmuck::PodCastError;
/// use constmuck::try_cast_ref_alt;
///
/// const U8: Result<&[u8; 2], PodCastError> = try_cast_ref_alt(&100u16.to_le());
/// const ERR_SIZE : Result<&u8, PodCastError> = try_cast_ref_alt(&100u16.to_le());
/// const ERR_ALIGN: Result<&u16, PodCastError> = try_cast_ref_alt(&100u8);
///
/// assert_eq!(U8, Ok(&[100u8, 0]));
/// assert_eq!(ERR_SIZE, Err(PodCastError::SizeMismatch));
/// assert_eq!(ERR_ALIGN, Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned));
///
/// ```
pub const fn try_cast_ref_alt<T, U>(from: &T) -> Result<&U, PodCastError>
where
    T: NoUninit,
    U: AnyBitPattern,
{
    unsafe {
        if mem::align_of::<T>() < mem::align_of::<U>() {
            Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)
        } else if mem::size_of::<T>() != mem::size_of::<U>() {
            Err(PodCastError::SizeMismatch)
        } else {
            // safety: the `_bounds` parameter guarantees that both `T` and `U`
            // contain no padding and are valid for all bitpatterns.
            //
            // They are both guaranteed the same size in this branch,
            // and T is at least as aligned as U.
            Ok(__priv_transmute_ref!(T, U, from))
        }
    }
}

/// Reads a `T`  out of a byte slice.
///
/// # Panics
///
/// This panics if `size_of::<T>() != bytes.len()`
///
/// # Example
///
/// ```rust
/// use constmuck::{pod_read_unaligned, AnyBitPattern};
///
/// #[repr(C)]
/// #[derive(Debug, PartialEq, Copy, Clone, AnyBitPattern)]
/// struct Foo(u16, u16);
///
/// const FOO: Foo = {
///     let number = u32::from_be_bytes([0xDe, 0x11, 0x0_B, 0x0b]);
///     pod_read_unaligned(&number.to_ne_bytes())
/// };
///
/// assert_eq!(FOO, Foo(0xB0b, 0xDe11));
///
/// ```
#[track_caller]
pub const fn pod_read_unaligned<T: AnyBitPattern>(bytes: &[u8]) -> T {
    match try_pod_read_unaligned(bytes) {
        Ok(x) => x,
        Err(PodCastError::SizeMismatch | _) => {
            crate::__priv_utils::unequal_bytes_size_panic(bytes.len(), core::mem::size_of::<T>())
        }
    }
}

/// Reads a `T`  out of a byte slice.
///
/// # Errors
///
/// This returns `Err(PodCastError::SizeMismatch)` if
/// `size_of::<T>() != bytes.len()`
///
// dunno what a good example for this would be
pub const fn try_pod_read_unaligned<T: AnyBitPattern>(bytes: &[u8]) -> Result<T, PodCastError> {
    if core::mem::size_of::<T>() == bytes.len() {
        // SAFETY: the slice is as large as `T`,
        //         and `Packed` does not have alignment requirements.
        let packed = unsafe { *bytes.as_ptr().cast::<Packed<T>>() };

        Ok(packed.0)
    } else {
        Err(PodCastError::SizeMismatch)
    }
}
