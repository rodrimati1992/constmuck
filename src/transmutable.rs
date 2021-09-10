//! Functions for safely transmuting types with a [`TransmutableInto`] parameter.
//!
//!

use core::{marker::PhantomData, mem};

use crate::{ImplsPod, __priv_utils::MakePhantom};

#[doc(no_inline)]
pub use crate::TransmutableInto;

pub(crate) mod transmutable_into {
    use super::*;

    /// Marker type which guarantees that `Fro` is safely transmutable into `To`,
    /// both by value and by reference (and other pointer types).
    ///
    /// Related: [`transmutable`](crate::transmutable) module.
    ///
    pub struct TransmutableInto<Fro, To> {
        _private: PhantomData<(
            // Makes this invariant over the lifetimes in `Fro` and `To`
            // so that it's not possible to change lifetime parameters.
            fn(PhantomData<Fro>) -> PhantomData<Fro>,
            fn(PhantomData<To>) -> PhantomData<To>,
        )>,
    }

    impl<Fro, To> Copy for TransmutableInto<Fro, To> {}
    impl<Fro, To> Clone for TransmutableInto<Fro, To> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<Fro, To> TransmutableInto<Fro, To> {
        /// Constructs a `TransmutableInto`
        ///
        /// # Safety
        ///
        /// `Fro` must be soundly convertible to `To`.
        ///
        /// Pointers to `Fro` must be soundly convertible to point to `To`,
        /// eg: transmuting `&Fro` to `&To`.
        ///
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self {
                _private: MakePhantom::MAKE,
            }
        }

        /// Constructs a `TransmutableInto`
        ///
        /// # Panics
        ///
        /// Panics if either:
        /// - The size of `Fro` isn't the same as `To`.
        /// - The alignment of `Fro` is less than `To`.
        ///
        #[inline(always)]
        pub const fn pod(_bounds: (ImplsPod<Fro>, ImplsPod<To>)) -> Self {
            if mem::size_of::<Fro>() != mem::size_of::<To>() {
                #[allow(non_snake_case)]
                let size_of_Foo = mem::size_of::<Fro>();
                [/* size of Foo != Bar */][size_of_Foo]
            } else if mem::align_of::<Fro>() != mem::align_of::<To>() {
                #[allow(non_snake_case)]
                let align_of_Foo = mem::align_of::<Fro>();
                [/* alingment of Foo != Bar */][align_of_Foo]
            } else {
                Self {
                    _private: MakePhantom::MAKE,
                }
            }
        }

        /// Turns a `TransmutableInto<Fro, To>` into a
        /// `TransmutableInto<[Fro; LEN], [To; LEN]>`.
        #[inline(always)]
        pub const fn array<const LEN: usize>(self) -> TransmutableInto<[Fro; LEN], [To; LEN]> {
            TransmutableInto {
                _private: MakePhantom::MAKE,
            }
        }
    }
}

/// Transmutes `T` into `U`, given a [`TransmutableInto`].
///
/// # Example
///
/// ```
/// use constmuck::{
///     transmutable::{TransmutableInto, transmute_into},
///     infer,
/// };
///
/// use std::num::NonZeroU8;
///
///
/// const X: [u8; 2] = transmute_into(
///     [None::<NonZeroU8>, NonZeroU8::new(10)],
///     TransmutableInto::pod(infer!()),
/// );
///
/// assert_eq!(X, [0u8, 10]);
///
/// ```
pub const fn transmute_into<T, U>(value: T, _bounds: TransmutableInto<T, U>) -> U {
    unsafe { __priv_transmute_unchecked!(T, U, value) }
}

/// Transmutes `&T` into `&U`, given a [`TransmutableInto`].
///
/// # Example
///
/// ```
/// use constmuck::{
///     transmutable::{TransmutableInto, transmute_ref},
///     infer,
/// };
///
/// use std::num::NonZeroU8;
///
///
/// const X: &[i8; 2] = transmute_ref(
///     &[None::<NonZeroU8>, NonZeroU8::new(255)],
///     TransmutableInto::pod(infer!()),
/// );
///
/// assert_eq!(*X, [0i8, -1]);
///
/// ```
pub const fn transmute_ref<T, U>(value: &T, _bounds: TransmutableInto<T, U>) -> &U {
    unsafe { __priv_transmute_ref_unchecked!(T, U, value) }
}

/// Transmutes `&[T]` into `&[U]`, given a [`TransmutableInto`].
///
/// # Example
///
/// ```
/// use constmuck::{
///     transmutable::{TransmutableInto, transmute_slice},
///     infer, infer_tw,
/// };
///
/// use std::num::Wrapping;
///
///
/// const X: &[i8] = transmute_slice(&[5u8, 250u8, 255u8], TransmutableInto::pod(infer!()));
///
/// // Casting a slice of `TransparentWrapper`s to a slice of what they wrap.
/// const UNWRAPPED: &[u8] =
///     transmute_slice(&[Wrapping(5), Wrapping(250)], infer_tw!().into_inner);
///
/// // Casting a slice of elements to a slice of `TransparentWrapper`s around them.
/// const WRAPPED: &[Wrapping<u8>] = transmute_slice(&[7, 78], infer_tw!().from_inner);
///
/// assert_eq!(*X, [5, -6, -1]);
/// assert_eq!(*UNWRAPPED, [5, 250]);
/// assert_eq!(*WRAPPED, [Wrapping(7), Wrapping(78)]);
///
/// ```
pub const fn transmute_slice<T, U>(value: &[T], _bounds: TransmutableInto<T, U>) -> &[U] {
    unsafe { __priv_transmute_ref_unchecked!([T], [U], value) }
}
