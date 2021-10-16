//! For copying values in generic contexts.
//!
//! Related: `IsCopy` [type](struct@crate::IsCopy), [macro](macro@crate::IsCopy),
//! and [`TypeSize`](macro@crate::TypeSize) macro.
#![allow(deprecated)]

use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::MaybeUninit,
};

use crate::TypeSize;

/// Constructs an [`IsCopy<$T>`](struct@crate::IsCopy),
/// requires `$T:`[`Pod`](trait@bytemuck::Pod) for reasons explained in the
/// [bound](struct@crate::IsCopy#bound) section.
///
/// This has an optional type argument (`$T`) that defaults to
/// infering the type if not passed.
///
/// This macro is defined for completeness' sake,
/// no function in this crate takes `IsCopy` by itself,
/// always a [`TypeSize<T, IsCopy<T>, _>`](struct@crate::TypeSize),
/// which can be constructed with the
/// [`TypeSize`](macro@crate::TypeSize) macro.
///
/// Related: the [`copying`](crate::copying) module
///
/// # Example
///
/// ```rust
/// use constmuck::{IsCopy, TypeSize, copying};
///
/// const FOO: IsCopy<u32> = IsCopy!();
/// assert_eq!(copying::copy_ts(&100u32, TypeSize!(u32).with_bounds(FOO)), 100);
/// // the typical way to call `copying::copy_ts`.
/// assert_eq!(copying::copy_ts(&100u32, TypeSize!(u32)), 100);
///
///
/// const BAR: IsCopy<[u8; 4]> = IsCopy!([u8; 4]);
/// let arr = [3, 5, 8, 13];
/// assert_eq!(copying::copy_ts(&arr, TypeSize!([u8; 4]).with_bounds(BAR)), arr);
/// // the typical way to call `copying::copy_ts`.
/// assert_eq!(copying::copy_ts(&arr, TypeSize!([u8; 4])), arr);
///
///
/// ```
#[macro_export]
macro_rules! IsCopy {
    () => {
        <$crate::IsCopy<_> as $crate::Infer>::INFER
    };
    ($T:ty) => {
        <$crate::IsCopy<$T> as $crate::Infer>::INFER
    };
}

pub(crate) mod is_copy {
    use super::*;

    /// Encodes that a `T` is trivially copyable.
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
    /// [`Pod`]: trait@bytemuck::Pod
    pub struct IsCopy<T> {
        // The lifetime of `T` is invariant,
        // just in case that it's unsound for lifetimes to be co/contravariant.
        _private: PhantomData<fn(T) -> T>,
    }

    impl<T> Debug for IsCopy<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("IsCopy")
        }
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
        /// You can also use the [`IsCopy`](macro@crate::IsCopy)
        /// macro to construct `IsCopy` arguments.
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
        /// The "no pointers" requirement is due to how [`constmuck::copying`]
        /// copies values, and might be lifted in the future.
        ///
        /// [`Pod`]: trait@bytemuck::Pod
        /// [`constmuck::copying`]: crate::copying
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
/// (see [`IsCopy`](struct@crate::IsCopy) docs for why it requires `Pod`)
///
/// # Suffix
///
/// This has a `_ts` suffix to allow eventually adding
/// ```rust
/// # use constmuck::IsCopy;
/// const fn copy<T>(reff: &T, bounds: IsCopy<T>) -> T
/// # {loop{}}
/// ```
/// when it's possible to implement without unstable features.
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
///     bounds: TypeSize<T, IsCopy<T>, SIZE>
/// ) -> [T; 2] {
///     [copying::copy_ts(reff, bounds), copying::copy_ts(reff, bounds)]
/// }
///
/// const PAIR_U8: [u8; 2] = pair(&128, TypeSize!(u8));
///
/// assert_eq!(PAIR_U8, [128, 128]);
///
/// ```
pub const fn copy_ts<T, const SIZE: usize>(reff: &T, bounds: TypeSize<T, IsCopy<T>, SIZE>) -> T {
    // safety:
    // `IsCopy<T>` guarantees that `T` is safe to copy using
    // an intermediate `MaybeUninit<[u8; std::mem::size_of::<T>()]>`,
    // by requiring `T: Pod`(the bound may change in the future).
    //
    // `TypeSize<T, _, SIZE>` guarantees that `T` is `SIZE` bytes large
    //
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
/// (see [`IsCopy`](struct@crate::IsCopy) docs for why it requires `Pod`)
///
/// To specify the length of the returned array,
/// [`TypeSize::repeat`] can be used instead.
///
/// # Suffix
///
/// This has a `_ts` suffix to allow eventually adding
/// ```rust
/// # use constmuck::IsCopy;
/// const fn repeat<T, const ARR_LEN: usize>(reff: &T, bounds: IsCopy<T>) -> [T; ARR_LEN]
/// # {loop{}}
/// ```
/// when it's possible to implement without unstable features.
///
/// # Example
///
/// ```rust
/// use constmuck::{TypeSize, copying};
///
/// const PAIR: [[u8; 2]; 2] = copying::repeat_ts(&[3, 5], TypeSize!([u8; 2]));
///
/// assert_eq!(PAIR, [[3, 5], [3, 5]]);
///
/// // you can use `TypeSize::repeat` like here to pass the length of the returned array.
/// assert_eq!(TypeSize!([u8; 2]).repeat::<2>(&[3, 5]), [[3, 5], [3, 5]]);
///
/// ```
pub const fn repeat_ts<T, const SIZE: usize, const ARR_LEN: usize>(
    reff: &T,
    bounds: TypeSize<T, IsCopy<T>, SIZE>,
) -> [T; ARR_LEN] {
    if crate::__priv_utils::SizeIsStride::<T, ARR_LEN>::V {
        crate::__priv_utils::SizeIsStride::<T, ARR_LEN>::panic();
    }

    // safety: same as `copying::copy_ts`
    unsafe {
        __priv_transmute_from_copy!(
            [MaybeUninit<[u8; SIZE]>; ARR_LEN],
            [T; ARR_LEN],
            [*crate::slice_fns::maybe_uninit_bytes_of(reff, bounds); ARR_LEN]
        )
    }
}
