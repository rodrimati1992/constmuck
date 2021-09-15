//! Functions for safely transmuting types with a [`TransmutableInto`] parameter.
//!
//!

use core::{marker::PhantomData, mem};

use crate::ImplsPod;

#[doc(no_inline)]
pub use crate::TransmutableInto;

pub(crate) mod transmutable_into {
    use super::*;

    /// Marker type which guarantees that `Fro` is safely transmutable into `To`,
    /// both by value and by (mutable) reference.
    ///
    /// Related: [`transmutable`](crate::transmutable) module.
    ///
    /// Functions for transmuting that require `align_of::<Fro>() == align_of::<To>()`
    /// (eg: a function that transmutes from `Arc<Fro>` to `Arc<To>`)
    /// have to contain that equality check as an assertion,
    /// because `TransmutableInto`'s constructors
    /// only require `align_of::<Fro>() >= align_of::<To>()`.
    ///
    /// # Example
    ///
    /// ```
    /// use constmuck::{
    ///     transmutable::{TransmutableInto, transmute_into, transmute_slice},
    ///     infer,
    /// };
    ///
    /// use std::num::Wrapping;
    ///
    /// // Transmuting from `&[u8]` to `&[i8]`
    /// const POD: &[i8] =
    ///     transmute_slice(&[5u8, 25, 125, 250], TransmutableInto::pod(infer!()));
    /// assert_eq!(*POD, [5, 25, 125, -6]);
    ///
    ///
    ///
    /// ```
    pub struct TransmutableInto<Fro: ?Sized, To: ?Sized> {
        _private: PhantomData<(
            // Makes this invariant over the lifetimes in `Fro` and `To`
            // so that it's not possible to change lifetime parameters.
            fn(PhantomData<Fro>) -> PhantomData<Fro>,
            fn(PhantomData<To>) -> PhantomData<To>,
        )>,
        #[doc(hidden)]
        pub _transmutable_into_proof: constmuck_internal::TransmutableProof<Fro, To>,
    }

    impl<Fro: ?Sized, To: ?Sized> Copy for TransmutableInto<Fro, To> {}
    impl<Fro: ?Sized, To: ?Sized> Clone for TransmutableInto<Fro, To> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<Fro: ?Sized, To: ?Sized> TransmutableInto<Fro, To> {
        const __NEW_UNCHECKED: Self = unsafe {
            Self {
                _private: PhantomData,
                _transmutable_into_proof: constmuck_internal::TransmutableProof::new_unchecked(),
            }
        };

        /// Constructs a `TransmutableInto`
        ///
        /// # Safety
        ///
        /// `Fro` must be soundly transmutable to `To`.
        ///
        /// References (`&` and `&mut`) to `Fro` must be soundly transmutable to point to `To`.
        ///
        /// `align_of::<Fro>()` must be greater than or equal to `align_of::<To>()`.
        ///
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self::__NEW_UNCHECKED
        }
    }

    impl<Fro, To> TransmutableInto<Fro, To> {
        /// Constructs a `TransmutableInto`
        ///
        /// # Panics
        ///
        /// Panics if either:
        /// - The size of `Fro` isn't the same as `To`.
        /// - The alignment of `Fro` is less than `To`
        /// (`Foo` is allowed to be more aligned than `To`).
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{
        ///     transmutable::{TransmutableInto, transmute_into},
        ///     infer,
        /// };
        ///
        /// {
        ///     // Transmuting from `[u8; 5]` to `[i8; 5]`
        ///     const POD: [i8; 5] = transmute_into(
        ///         [0u8, 127, 128, 129, 130],
        ///         TransmutableInto::pod(infer!()),
        ///     );
        ///    
        ///     assert_eq!(POD, [0, 127, -128, -127, -126]);
        /// }
        /// ```
        #[inline(always)]
        pub const fn pod(_bounds: (ImplsPod<Fro>, ImplsPod<To>)) -> Self {
            if mem::size_of::<Fro>() != mem::size_of::<To>() {
                #[allow(non_snake_case)]
                let size_of_Foo = mem::size_of::<Fro>();
                [/* size of Foo != Bar */][size_of_Foo]
            } else if mem::align_of::<Fro>() < mem::align_of::<To>() {
                #[allow(non_snake_case)]
                let align_of_Foo = mem::align_of::<Fro>();
                [/* alignment of Foo < Bar */][align_of_Foo]
            } else {
                Self::__NEW_UNCHECKED
            }
        }

        /// Turns a `TransmutableInto<Fro, To>` into a
        /// `TransmutableInto<[Fro; LEN], [To; LEN]>`.
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{
        ///     transmutable::{TransmutableInto, transmute_ref},
        ///     infer_tw,
        /// };
        ///
        /// #[derive(Debug, PartialEq)]
        /// #[repr(transparent)]
        /// pub struct Other<T>(pub T);
        ///
        /// unsafe impl<T> constmuck::TransparentWrapper<T> for Other<T> {}
        ///
        /// {
        ///     // Transmuting from `&[Other<u32>; 5]` to `&[u32; 5]`
        ///     const ARR: &[u32; 5] = transmute_ref(
        ///         &[Other(0), Other(127), Other(128), Other(129), Other(130)],
        ///         // `infer_tw!().into_inner.array()` allows transmuting an arrays of wrappers
        ///         // into an arraw of the values that are inside those wrappers.
        ///         infer_tw!().into_inner.array(),
        ///     );
        ///    
        ///     assert_eq!(*ARR, [0, 127, 128, 129, 130]);
        /// }
        /// ```
        #[inline(always)]
        pub const fn array<const LEN: usize>(self) -> TransmutableInto<[Fro; LEN], [To; LEN]> {
            TransmutableInto::__NEW_UNCHECKED
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
///     infer, infer_tw,
/// };
///
/// use std::num::NonZeroU8;
///
/// {
///     // Transmuting from `[Option<NonZeroU8>; 2]` to `[u8; 2]`
///     const POD: [u8; 2] = transmute_into(
///         [None::<NonZeroU8>, NonZeroU8::new(10)],
///         TransmutableInto::pod(infer!()),
///     );
///    
///     assert_eq!(POD, [0u8, 10]);
/// }
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct That<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for That<T> {}
///
/// {
///     // Transmuting from `[char; 4]` to `That<[char; 4]>`
///     //
///     // `infer_tw!()` constructs an `ImplsTransparentWrapper`,
///     // whose `from_inner` field allows transmuting from a value into a wrapper around it.
///     const THAT_ARRAY: That<[char; 4]> =
///         transmute_into(['A', 'E', 'I', 'O'], infer_tw!().from_inner);
///     assert_eq!(THAT_ARRAY, That(['A', 'E', 'I', 'O']));
/// }
///
/// {
///     // Transmuting from `[That<char>; 4]` to `[char; 4]`
///     //
///     // `infer_tw!().into_inner.array()` allows transmuting an arrays of wrappers
///     // into an arraw of the values that are inside those wrappers.
///     const ARRAY_THAT: [char; 4] = transmute_into(
///         [That('A'), That('E'), That('I'), That('O')],
///         infer_tw!().into_inner.array(),
///     );
///     assert_eq!(ARRAY_THAT, ['A', 'E', 'I', 'O']);
/// }
///
///
/// ```
pub const fn transmute_into<T, U>(value: T, _bounds: TransmutableInto<T, U>) -> U {
    unsafe { __priv_transmute!(T, U, value) }
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
/// // Transmuting from `[Option<NonZeroU8>; 2]` to `[i8; 2]`
/// const X: &[i8; 2] = transmute_ref(
///     &[None::<NonZeroU8>, NonZeroU8::new(255)],
///     TransmutableInto::pod(infer!()),
/// );
///
/// assert_eq!(*X, [0i8, -1]);
///
/// ```
pub const fn transmute_ref<T, U>(value: &T, _bounds: TransmutableInto<T, U>) -> &U {
    unsafe { __priv_transmute_ref!(T, U, value) }
}

/// Transmutes `&T` into `&U`, given a [`TransmutableInto`],
/// allows transmuting between `?Sized` types.
///
/// # Example
///
/// ```
/// use constmuck::{
///     transmutable::transmute_ref,
///     infer_tw,
/// };
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Wrapper<T: ?Sized>(pub T);
///
/// unsafe impl<T: ?Sized> constmuck::TransparentWrapper<T> for Wrapper<T> {}
///
/// // Transmuting from `&[u8]` to `&Wrapper<[u8]>`
/// const BYTES: &Wrapper<[u8]> = transmute_ref!(
///     b"hello" as &[u8],
///     infer_tw!().from_inner,
/// );
///
/// assert_eq!(&BYTES.0, b"hello");
///
/// ```
#[doc(inline)]
pub use constmuck_internal::transmute_ref;

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
/// // Transmuting from `&[u8]` to `&[i8]`
/// const SIGNED: &[i8] = transmute_slice(&[5u8, 250u8, 255u8], TransmutableInto::pod(infer!()));
/// assert_eq!(*SIGNED, [5, -6, -1]);
///
/// // Transmuting from `&[Wrapping<u8>]` to `&[i8]`
/// //
/// // `infer_tw!()` constructs an `ImplsTransparentWrapper`,
/// // whose `into_inner` field allows transmuting from a wrapper into the value in it.
/// const UNWRAPPED: &[u8] =
///     transmute_slice(&[Wrapping(5), Wrapping(250)], infer_tw!().into_inner);
/// assert_eq!(*UNWRAPPED, [5, 250]);
///
/// // Transmuting from `&[u8]` to `&[Wrapping<u8>]`
/// //
/// // `infer_tw!()` constructs an `ImplsTransparentWrapper`,
/// // whose `from_inner` field allows transmuting from a value into a wrapper around it.
/// const WRAPPED: &[Wrapping<u8>] = transmute_slice(&[7, 78], infer_tw!().from_inner);
/// assert_eq!(*WRAPPED, [Wrapping(7), Wrapping(78)]);
///
///
/// ```
pub const fn transmute_slice<T, U>(value: &[T], _bounds: TransmutableInto<T, U>) -> &[U] {
    unsafe { __priv_transmute_slice!(T, U, value) }
}
