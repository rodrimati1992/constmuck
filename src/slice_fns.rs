use core::mem;

use bytemuck::{AnyBitPattern, NoUninit, PodCastError};

/// Casts `&T` to `&[u8; SIZE]`
///
/// # Example
///
/// ```rust
/// use constmuck::bytes_of;
///
/// const BYTES: &[u8] = bytes_of(&987654321u32);
///
/// assert_eq!(*BYTES, 987654321u32.to_ne_bytes());
/// ```
pub const fn bytes_of<T>(bytes: &T) -> &[u8]
where
    T: NoUninit,
{
    // safety: `T: NoUninit` guarantees that T doesn't have any padding or uninit bytes,
    unsafe {
        core::slice::from_raw_parts(bytes as *const T as *const u8, core::mem::size_of::<T>())
    }
}

/// Casts `&[T]` to `&[U]`
///
/// # Panics
///
/// This function panics in the cases where [`try_cast_slice_alt`]
/// returns [an error](crate::try_cast_slice_alt#errors).
///
/// # Difference with `bytemuck`
///
/// This function has [the same differences](crate::try_cast_slice_alt#differences)
/// with [`bytemuck::cast_slice`]
/// that [`try_cast_slice_alt`] does  with
/// [`bytemuck::try_cast_slice`].
///
/// # Example
///
/// ```
/// use constmuck::PodCastError;
/// use constmuck::cast_slice_alt;
///
/// type Res<T> = Result<T, PodCastError>;
///
/// const I8S: &[i8] = cast_slice_alt(&[100u8, 254, 255]);
///
/// assert_eq!(*I8S, [100, -2, -1]);
///
/// ```
pub const fn cast_slice_alt<T, U>(from: &[T]) -> &[U]
where
    T: NoUninit,
    U: AnyBitPattern,
{
    match try_cast_slice_alt(from) {
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

/// Tries to cast `&[T]` to `&[U]`
///
/// # Errors
///
/// This function returns errors in these cases:
/// - The alignment of `T` is larger than `U`, returning a
/// `Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)`.
/// <br>(using this instead of `PodCastError::AlignmentMismatch` because that
/// is not returned by [`bytemuck::try_cast_slice`])
///
/// - The size of `T` is not equal to `U`, returning a `Err(PodCastError::SizeMismatch)`.
///
/// <span id="differences"></span>
/// # Difference with `bytemuck`
///
/// This function requires `T` to have an alignment larger or equal to `U`,
/// while [`bytemuck::try_cast_slice`] only requires the `from` reference
/// to happen to be aligned to `U`.
///
/// [`bytemuck::try_cast_slice`] allows the size of `T` to be different than `U` if
/// it divides evenly into it, this function does not due to limitations in stable const fns.
///
/// # Example
///
/// ```
/// use constmuck::PodCastError;
/// use constmuck::try_cast_slice_alt;
///
/// type Res<T> = Result<T, PodCastError>;
///
/// const I8S: Res<&[i8]> = try_cast_slice_alt(&[100u8, 254, 255]);
/// const ERR_SIZE : Res<&[u8]> = try_cast_slice_alt(&[0u16]);
///
/// assert_eq!(I8S, Ok(&[100i8, -2, -1][..]));
/// assert_eq!(ERR_SIZE, Err(PodCastError::SizeMismatch));
///
/// ```
pub const fn try_cast_slice_alt<T, U>(from: &[T]) -> Result<&[U], PodCastError>
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
            Ok(__priv_transmute_slice!(T, U, from))
        }
    }
}
