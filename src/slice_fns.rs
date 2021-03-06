use core::mem::{self, MaybeUninit};

use bytemuck::PodCastError;

use crate::{IsCopy, IsPod, TypeSize};

/// Casts `&T` to `&[u8; SIZE]`
///
/// Requires `T` to implement [`Pod`](trait@bytemuck::Pod).
///
/// `SIZE` is guaranteed to be the size of `T` by the `TypeSize` argument.
///
/// # Example
///
/// ```rust
/// use constmuck::{TypeSize, byte_array_of};
///
/// const ARRAY: &[u8; 4] = byte_array_of(&123456789, TypeSize!(u32));
/// const BYTES: &[u8] = byte_array_of(&987654321, TypeSize!(u32));
///
/// assert_eq!(*ARRAY, 123456789u32.to_ne_bytes());
/// assert_eq!(*BYTES, 987654321u32.to_ne_bytes());
/// ```
pub const fn byte_array_of<T, const SIZE: usize>(
    bytes: &T,
    _bounds: TypeSize<T, IsPod<T>, SIZE>,
) -> &[u8; SIZE] {
    // safety:
    // `TypeSize` guarantees that `size_of::<T>() == SIZE`
    //
    // `IsPod` guarantees that the type doesn't have any padding,
    // and allows any bit pattern,
    unsafe { __priv_transmute_ref!(T, [u8; SIZE], bytes) }
}

// Internal helper function for use in copying a Copy type.
//
// Once it's possible to copy generic types without using an intermediate
// `MaybeUninit<[u8; SIZE]>` this function will be deleted.
pub(crate) const fn maybe_uninit_bytes_of<T, const SIZE: usize>(
    bytes: &T,
    _bounds: TypeSize<T, IsCopy<T>, SIZE>,
) -> &MaybeUninit<[u8; SIZE]> {
    // safety:
    // `IsCopy<T>` guarantees that `T` is safe to copy using
    // an intermediate `MaybeUninit<[u8; std::mem::size_of::<T>()]>`.
    //
    // `TypeSize<T, _, SIZE>` guarantees that `T` is `SIZE` bytes large
    //
    unsafe { __priv_transmute_ref!(T, MaybeUninit<[u8; SIZE]>, bytes) }
}

/// Casts `&[T]` to `&[U]`
///
/// Requires both `T` and `U` to implement [`Pod`](trait@bytemuck::Pod).
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
/// use constmuck::{cast_slice_alt, infer};
///
/// type Res<T> = Result<T, PodCastError>;
///
/// const I8S: &[i8] = cast_slice_alt(&[100u8, 254, 255], infer!());
///
/// assert_eq!(*I8S, [100, -2, -1]);
///
/// ```
pub const fn cast_slice_alt<T, U>(from: &[T], bounds: (IsPod<T>, IsPod<U>)) -> &[U] {
    match try_cast_slice_alt(from, bounds) {
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
/// Requires both `T` and `U` to implement [`Pod`](trait@bytemuck::Pod).
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
/// use constmuck::{infer, try_cast_slice_alt};
///
/// type Res<T> = Result<T, PodCastError>;
///
/// const I8S: Res<&[i8]> = try_cast_slice_alt(&[100u8, 254, 255], infer!());
/// const ERR_SIZE : Res<&[u8]> = try_cast_slice_alt(&[0u16], infer!());
///
/// assert_eq!(I8S, Ok(&[100i8, -2, -1][..]));
/// assert_eq!(ERR_SIZE, Err(PodCastError::SizeMismatch));
///
/// ```
pub const fn try_cast_slice_alt<T, U>(
    from: &[T],
    _bounds: (IsPod<T>, IsPod<U>),
) -> Result<&[U], PodCastError> {
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
