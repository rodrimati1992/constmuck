use core::mem::{align_of, size_of};

use bytemuck::{AnyBitPattern, NoUninit, PodCastError};

/// Casts `&T` to `&[u8]`
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
    unsafe { core::slice::from_raw_parts(bytes as *const T as *const u8, size_of::<T>()) }
}

/// Casts `&[T]` to `&[U]`
///
/// If this function does not panic,
/// the length of the returned slice is `from.len() * size_of::<T>() / size_of::<U>()`.
///
/// # Panics
///
/// This function panics in the cases where [`try_cast_slice_alt`]
/// returns [an error](crate::try_cast_slice_alt#errors).
///
/// # Difference with `bytemuck`
///
/// This function requires `T` to have an alignment larger than or equal to `U`.
/// [`bytemuck::cast_slice`] allows `T` to have a lower alignment than `U`,
/// so long as the `from` reference happens to be aligned to `U`.
///
/// # Example
///
/// ```
/// use constmuck::cast_slice_alt;
///
/// const TRIPLES: &[[u8; 3]] = cast_slice_alt(&[3u8, 5, 8, 13, 21, 34, 55, 89, 144]);
///
/// assert_eq!(*TRIPLES, [[3, 5, 8], [13, 21, 34], [55, 89, 144]]);
///
/// ```
#[track_caller]
pub const fn cast_slice_alt<T, U>(from: &[T]) -> &[U]
where
    T: NoUninit,
    U: AnyBitPattern,
{
    match try_cast_slice_alt(from) {
        Ok(x) => x,
        Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned) => {
            crate::__priv_utils::incompatible_alignment_panic(align_of::<T>(), align_of::<U>())
        }
        Err(PodCastError::OutputSliceWouldHaveSlop) => {
            crate::__priv_utils::slice_does_not_divide_evenly_panic(
                from.len(),
                size_of::<T>(),
                size_of::<U>(),
            )
        }
        Err(PodCastError::SizeMismatch) => {
            crate::__priv_utils::slice_cast_zst_panic(size_of::<T>(), size_of::<U>())
        }
        Err(PodCastError::AlignmentMismatch) => {
            // can't use `unreachable` macro in const fn
            panic!("unreachable!")
        }
    }
}

/// Tries to cast `&[T]` to `&[U]`
///
/// If this function returns successfully,
/// the length of the returned slice is `from.len() * size_of::<T>() / size_of::<U>()`.
///
/// # Errors
///
/// This function returns errors in these cases:
/// - The alignment of `T` is larger than `U`, returning a
/// `Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)`.
///
/// - `T` xor `U` is zero-sized, but the other type parameter isn't zero-sized,
/// returning a `Err(PodCastError::SizeMismatch)`.
///
/// - `from.len() * size_of::<T>()` does not divide evenly into `size_of::<U>()`,
/// returning a `Err(PodCastError::OutputSliceWouldHaveSlop)`.
///
///
/// <span id="differences"></span>
/// # Difference with `bytemuck`
///
/// This function requires `T` to have an alignment larger than or equal to `U`.
/// [`bytemuck::try_cast_slice`] allows `T` to have a lower alignment than `U`,
/// so long as the `from` reference happens to be aligned to `U`.
///
/// # Example
///
/// ```
/// use constmuck::PodCastError;
/// use constmuck::try_cast_slice_alt;
///
/// type Res<T> = Result<T, PodCastError>;
///
/// // casting the slice to element of different length,
/// // this works as long as the slice's size divides evenly into the new element size.
/// const I8_PAIRS: Res<&[[i8; 2]]> = try_cast_slice_alt(&[100u8, 101, 102, 253, 254, 255]);
/// assert_eq!(I8_PAIRS, Ok(&[[100i8, 101], [102, -3], [-2, -1]][..]));
///
/// // this function can't be used to cast slices from ZSTs to non-ZSTs and vice versa.
/// const ERR_ZST: Res<&[()]> = try_cast_slice_alt(&[0u8]);
/// assert_eq!(ERR_ZST, Err(PodCastError::SizeMismatch));
///
/// // this produces an error, since the slice's size (in bytes) does not
/// // divide evenly into `[u8; 2]`'s size.
/// const ERR_SLOP: Res<&[[u8; 2]]> = try_cast_slice_alt(&[3u8, 5, 8]);
/// assert_eq!(ERR_SLOP, Err(PodCastError::OutputSliceWouldHaveSlop));
///
/// // this produces an error because the element's alignment is increased.
/// const ERR_ALIGN: Res<&[u16]> = try_cast_slice_alt(&[3u8, 5, 8, 13]);
/// assert_eq!(ERR_ALIGN, Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned));
///
///
/// ```
pub const fn try_cast_slice_alt<T, U>(from: &[T]) -> Result<&[U], PodCastError>
where
    T: NoUninit,
    U: AnyBitPattern,
{
    // SAFETY for both unsafe blocks:
    // `T: NoUninit` guarantees that `T` contains no uninitialized bytes,
    // `U: AnyBitPattern` guarantees that U is valid for any bit pattern.
    // both bounds combined mean that transmuting between them is safe,
    // provided that their size/alignment is compatible.
    if align_of::<T>() < align_of::<U>() {
        Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)
    } else if size_of::<T>() == size_of::<U>() {
        // T is at least as aligned as U, and is the same size as U.
        unsafe { Ok(__priv_transmute_slice! {T, U, from}) }
    } else if size_of::<T>() == 0 || size_of::<U>() == 0 {
        Err(PodCastError::SizeMismatch)
    } else if let Some(new_len) = exact_div(from.len() * size_of::<T>(), size_of::<U>()) {
        // T is at least as aligned as U.
        //
        // (size_of::<T>() * from.len()) divides evenly into size_of::<U>()
        unsafe {
            Ok(core::slice::from_raw_parts(
                from.as_ptr().cast::<U>(),
                new_len,
            ))
        }
    } else {
        Err(PodCastError::OutputSliceWouldHaveSlop)
    }
}

// Returns dividend / divisor iff the division has no remainder,
// otherwise returns None.
//
// Panics if `divisor == 0`.
const fn exact_div(dividend: usize, divisor: usize) -> Option<usize> {
    if dividend % divisor == 0 {
        Some(dividend / divisor)
    } else {
        None
    }
}
