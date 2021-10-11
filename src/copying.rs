//! For copying values in generic contexts.
//!
//!
//! Related: [`IsCopy`] type, [`TypeSize`](macro@crate::TypeSize) macro.
#![allow(deprecated)]

use core::{marker::PhantomData, mem::MaybeUninit};

use crate::TypeSize;

pub(crate) mod is_copy {
    use super::*;

    /// Encodes that a `T` is trivially copyable,
    /// avoiding requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// Related: the [`copying`](crate::copying) module
    ///
    /// # Bound
    ///
    /// This currently requires [`Pod`]
    /// because the approach constmuck has for copying uses
    /// an intermediate `MaybeUninit<[u8; N]>`,
    /// which isn't settled as being sound to use with pointer types,
    /// but is guaranteed sound for [`Pod`] types.
    ///
    /// If there's a more permissive bound that allows more non-pointer-containing
    /// `Copy` types, `IsCopy` will be changed to use that.
    ///
    /// [`Pod`]: bytemuck::Pod
    pub struct IsCopy<T> {
        _private: PhantomData<fn() -> T>,
    }

    impl<T> Copy for IsCopy<T> {}

    impl<T> Clone for IsCopy<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: crate::Pod> IsCopy<T> {
        /// Constructs an `IsCopy`
        ///
        /// You can also use the [`infer`] macro to construct `IsCopy` arguments.
        pub const NEW: Self = Self {
            _private: PhantomData,
        };
    }

    impl<T> IsCopy<T> {
        const __NEW_UNCHECKED__: Self = Self {
            _private: PhantomData,
        };

        /// Constructs an `IsCopy<T>` without checking that `T`
        /// implements [`Copy`] + [`Pod`].
        ///
        /// # Safety
        ///
        /// You must ensure that `T` is safe to `memcpy` without forgetting the
        /// copied-from value, and doesn't contain pointers of any kind.
        ///
        /// [`Pod`]: bytemuck::Pod
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self::__NEW_UNCHECKED__
        }
    }
}

#[doc(no_inline)]
pub use crate::IsCopy;

impl<T: crate::Pod> crate::Infer for IsCopy<T> {
    const INFER: Self = Self::NEW;
}

/// Copies a `T` from a `&T`
///
/// Requires that `T` implements `Copy + Pod`
/// (see [`IsCopy`] docs for why it requires `Pod`)
///
/// # Example
///
/// Making a `pair` function.
///
/// ```rust
/// use constmuck::copying;
/// use constmuck::{IsCopy, TypeSize};
///
/// const fn pair<T, const SIZE: usize>(
///     reff: &T,
///     bounds: TypeSize<IsCopy<T>, T, SIZE>
/// ) -> [T; 2] {
///     [copying::copy(reff, bounds), copying::copy(reff, bounds)]
/// }
///
/// const PAIR_U8: [u8; 2] = pair(&128, TypeSize!(u8));
///
/// assert_eq!(PAIR_U8, [128, 128]);
///
/// ```
pub const fn copy<T, const SIZE: usize>(reff: &T, bounds: TypeSize<IsCopy<T>, T, SIZE>) -> T {
    unsafe {
        __priv_transmute_from_copy!(
            MaybeUninit<[u8; SIZE]>,
            T,
            *crate::slice_fns::maybe_uninit_bytes_of(reff, bounds)
        )
    }
}

/// Creates a `[T; ARR_LEN]` by copying from a `&T`
///
/// Requires that `T` implements `Copy + Pod`
/// (see [`IsCopy`] docs for why it requires `Pod`)
///
/// To specify the length of the returned array,
/// [`TypeSize::repeat`] can be used instead.
///
/// # Example
///
/// ```rust
/// use constmuck::{TypeSize, copying};
///
/// const PAIR: [[u8; 2]; 2] = copying::repeat(&[3, 5], TypeSize!([u8; 2]));
///
/// assert_eq!(PAIR, [[3, 5], [3, 5]]);
///
/// // you can use `TypeSize::repeat` like here to pass the length of the returned array.
/// assert_eq!(TypeSize!([u8; 2]).repeat::<2>(&[3, 5]), [[3, 5], [3, 5]]);
///
/// ```
pub const fn repeat<T, const SIZE: usize, const ARR_LEN: usize>(
    reff: &T,
    bounds: TypeSize<IsCopy<T>, T, SIZE>,
) -> [T; ARR_LEN] {
    unsafe {
        __priv_transmute_from_copy!(
            [MaybeUninit<[u8; SIZE]>; ARR_LEN],
            [T; ARR_LEN],
            [*crate::slice_fns::maybe_uninit_bytes_of(reff, bounds); ARR_LEN]
        )
    }
}
