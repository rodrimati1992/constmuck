use core::mem::{self, MaybeUninit};

use crate::{ImplsCopy, ImplsPod, TypeSize};

/// Casts `&T` to `&[u8; SIZE]`
///
/// `SIZE` is guaranteed to be the size of `T` by the `TypeSize` argument.
///
/// # Example
///
/// ```rust
/// use constmuck::{bytes_of, type_size};
///
/// const ARRAY: &[u8; 4] = bytes_of(&123456789, type_size!(u32));
/// const BYTES: &[u8] = bytes_of(&987654321, type_size!(u32));
///
/// assert_eq!(*ARRAY, 123456789u32.to_ne_bytes());
/// assert_eq!(*BYTES, 987654321u32.to_ne_bytes());
/// ```
pub const fn bytes_of<T, const SIZE: usize>(
    bytes: &T,
    _bounds: TypeSize<ImplsPod<T>, T, SIZE>,
) -> &[u8; SIZE] {
    unsafe { __priv_transmute_ref_unchecked!(T, [u8; SIZE], bytes) }
}

pub(crate) const fn maybe_uninit_bytes_of<T, const SIZE: usize>(
    bytes: &T,
    _bounds: TypeSize<ImplsCopy<T>, T, SIZE>,
) -> &MaybeUninit<[u8; SIZE]> {
    unsafe { __priv_transmute_ref_unchecked!(T, MaybeUninit<[u8; SIZE]>, bytes) }
}
