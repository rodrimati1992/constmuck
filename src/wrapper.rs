//! Functions for wrapping/peeling types that implement [`TransparentWrapper`]
//!
//! Related: [`ImplsTransparentWrapper`] type and [`infer_tw`] macro.
//!
//! # Example
//!
//! Tranmuting between arrays of values and arrays of wrappers.
//!
//! ```rust
//! use constmuck::{infer_tw, wrapper};
//!
//! #[derive(Debug, PartialEq)]
//! #[repr(transparent)]
//! pub struct Foo<T>(pub T);
//!
//! unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
//!
//! // `[u8; 3]` to `[Foo<u8>; 3]`
//! {
//!     const ARR: [Foo<u8>; 3] = wrapper::wrap([3, 5, 8], infer_tw!().array());
//!
//!     assert_eq!(ARR, [Foo(3), Foo(5), Foo(8)]);
//!     // `infer_tw!(Foo<_>)` is required because any type can implement comparison with `Foo`.
//!     assert_eq!(
//!         wrapper::wrap([13, 21, 34], infer_tw!(Foo<_>).array()),
//!         [Foo(13), Foo(21), Foo(34)],
//!     );
//! }
//!
//! // `[Foo<u8>; 3]` to `[u8; 3]`
//! {
//!     const ARR: [u8; 3] = wrapper::peel([Foo(3), Foo(5), Foo(8)], infer_tw!().array());
//!
//!     assert_eq!(ARR, [3, 5, 8]);
//!     assert_eq!(
//!         wrapper::peel([Foo(13), Foo(21), Foo(34)], infer_tw!(Foo<_>).array()),
//!         [13, 21, 34],
//!     );
//! }
//!
//! // `&[u8; 3]` to `&[Foo<u8>; 3]`
//! {
//!     const REF_ARR: &[Foo<u8>; 3] = wrapper::wrap_ref(&[3, 5, 8], infer_tw!().array());
//!
//!     assert_eq!(REF_ARR, &[Foo(3), Foo(5), Foo(8)]);
//!     assert_eq!(
//!         wrapper::wrap_ref(&[13, 21, 34], infer_tw!(Foo<_>).array()),
//!         &[Foo(13), Foo(21), Foo(34)],
//!     );    
//! }
//!
//! // `&[Foo<u8>; 3]` to `&[u8; 3]`
//! {
//!     const REF_ARR: &[u8; 3] = wrapper::peel_ref(&[Foo(3), Foo(5), Foo(8)], infer_tw!().array());
//!
//!     assert_eq!(REF_ARR, &[3, 5, 8]);
//!     assert_eq!(
//!         wrapper::peel_ref(&[Foo(13), Foo(21), Foo(34)], infer_tw!(Foo<_>).array()),
//!         &[13, 21, 34],
//!     );    
//! }
//!
//! ```
//!

use bytemuck::TransparentWrapper;

use crate::{Infer, TransmutableInto};

#[doc(no_inline)]
pub use crate::{infer_tw, ImplsTransparentWrapper};

/// Constructs an [`ImplsTransparentWrapper`],
///
/// Most useful over [`infer`] to:
/// - Call methods on the [`ImplsTransparentWrapper`].
/// - Access the [`from_inner`](ImplsTransparentWrapper::from_inner) field
/// - Access the [`into_inner`](ImplsTransparentWrapper::into_inner) field.
///
/// Related: [`wrapper`](crate::wrapper) module
///
/// # Example
///
/// ### `constmuck::wrapper` functions
///
/// ```rust
/// use constmuck::{infer_tw, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// {
///     // Transmute `ì8` to `Foo<i8>`
///     const WRAP_VAL_ONE: Foo<i8> = wrapper::wrap(3, infer_tw!());
///     
///     // Transmute `[u8; 3]` to `[Foo<u8>; 3]`
///     //
///     // The `.array()` is required to transmute arrays of values into arrays of
///     // wrappers around those values.
///     const WRAP_VAL_ARR: [Foo<u8>; 3] = wrapper::wrap([5, 8, 13], infer_tw!().array());
///     
///     assert_eq!(WRAP_VAL_ONE, Foo(3));
///     assert_eq!(wrapper::wrap(3, infer_tw!(Foo<i8>)), Foo(3));
///     assert_eq!(wrapper::wrap(3, infer_tw!(Foo<i8>, i8)), Foo(3));
///     
///     assert_eq!(WRAP_VAL_ARR, [Foo(5), Foo(8), Foo(13)]);
///     assert_eq!(
///         wrapper::wrap([5, 8, 13], infer_tw!(Foo<u8>).array()),
///         [Foo(5), Foo(8), Foo(13)],
///     );
///     assert_eq!(
///         wrapper::wrap([5, 8, 13], infer_tw!(Foo<u8>, u8).array()),
///         [Foo(5), Foo(8), Foo(13)],
///     );
/// }
/// {
///     // Transmute `&i8` to `&Foo<i8>`
///     const WRAP_REF_ONE: &Foo<i8> = wrapper::wrap_ref(&3, infer_tw!());
///     
///     // Transmute `&[u8; 3]` to `&[Foo<u8>; 3]`
///     const WRAP_REF_ARR: &[Foo<u8>; 3] = wrapper::wrap_ref(&[5, 8, 13], infer_tw!().array());
///     
///     assert_eq!(WRAP_REF_ONE, &Foo(3));
///     assert_eq!(WRAP_REF_ARR, &[Foo(5), Foo(8), Foo(13)]);
/// }
/// {
///     // Transmute `&[i8]` to `&[Foo<i8>]`
///     const WRAP_SLICE: &[Foo<i8>] = wrapper::wrap_slice(&[21, 34, 55], infer_tw!());
///     
///     assert_eq!(WRAP_SLICE, &[Foo(21), Foo(34), Foo(55)]);
/// }
/// ```
///
/// ### `constmuck::transmutable` functions
///
/// ```
/// use constmuck::{
///     transmutable::transmute_ref,
///     infer_tw,
/// };
///
/// use std::num::Wrapping;
///
/// {
///     // Transmuting `&Wrapping<u8>` to `&u8`,
///     //
///     // `infer_tw!()` constructs an `ImplsTransparentWrapper`,
///     // whose `into_inner` field allows transmuting from a wrapper into the value in it.
///     const UNWRAPPED: &u8 = transmute_ref(&Wrapping(5), infer_tw!().into_inner);
///     assert_eq!(*UNWRAPPED, 5);
/// }
///
/// {
///     // Transmuting `&u8` to `&Wrapping<u8>`
///     //
///     // `infer_tw!()` constructs an `ImplsTransparentWrapper`,
///     // whose `from_inner` field allows transmuting from a value into a wrapper around it.
///     const WRAPPED: &Wrapping<u8> = transmute_ref(&7, infer_tw!().from_inner);
///    
///     assert_eq!(*WRAPPED, Wrapping(7));
/// }
///
/// ```
#[macro_export]
macro_rules! infer_tw {
    () => {
        <$crate::ImplsTransparentWrapper<_, _> as $crate::Infer>::INFER
    };
    ($outer:ty $(,)*) => {
        <$crate::ImplsTransparentWrapper<$outer, _> as $crate::Infer>::INFER
    };
    ($outer:ty, $inner:ty $(,)*) => {
        <$crate::ImplsTransparentWrapper<$outer, $inner> as $crate::Infer>::INFER
    };
}

pub(crate) mod impls_tw {
    use super::*;

    /// Encodes a `T:`[`TransparentWrapper`] bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// Constructible with [`NEW`](Self::NEW) associated constant,
    /// or [`infer_tw`] macro.
    ///
    /// Related: [`wrapper`](crate::wrapper) module.
    ///
    /// # Example
    ///
    /// ```rust
    /// use constmuck::{infer_tw, wrapper};
    ///
    /// #[derive(Debug, PartialEq)]
    /// #[repr(transparent)]
    /// pub struct This<T>(pub T);
    ///
    /// unsafe impl<T> constmuck::TransparentWrapper<T> for This<T> {}
    ///
    /// {
    ///     // Transmuting from `&&str` to `&This<&str>`
    ///     //
    ///     // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
    ///     const WRAPPED: &This<&str> = wrapper::wrap_ref(&"hi", infer_tw!());
    ///     assert_eq!(*WRAPPED, This("hi"));
    /// }
    ///
    /// {
    ///     // Transmuting from `&This<&str>` to `&&str`
    ///     const UNWRAPPED: &&str = wrapper::peel_ref(&This("hello"), infer_tw!());
    ///     assert_eq!(*UNWRAPPED, "hello");
    /// }
    ///
    /// {
    ///     // Transmuting from `[u64; 2]` to `[This<u64>; 2]`
    ///     //
    ///     // The `.array()` is required to transmute arrays of values into arrays of
    ///     // wrappers around those values.
    ///     const WRAPPED_ARR: [This<u64>; 2] = wrapper::wrap([9, 99], infer_tw!().array());
    ///     assert_eq!(WRAPPED_ARR, [This(9), This(99)]);
    /// }
    ///
    /// {
    ///     // Transmuting from `[This<i8>; 2]` to `[i8; 2]`
    ///     //
    ///     // `.array()` also allows transmuting arrays of wrappers into
    ///     // arrays of the values inside those wrappers, using the `peel*` functions.
    ///     const UNWRAPPED_ARR: [i8; 2] =
    ///         wrapper::peel([This(2), This(22)], infer_tw!().array());
    ///     assert_eq!(UNWRAPPED_ARR, [2, 22]);
    /// }
    ///
    /// ```
    pub struct ImplsTransparentWrapper<Outer: ?Sized, Inner: ?Sized> {
        pub from_inner: TransmutableInto<Inner, Outer>,
        pub into_inner: TransmutableInto<Outer, Inner>,
    }

    impl<Outer: ?Sized, Inner: ?Sized> Copy for ImplsTransparentWrapper<Outer, Inner> {}

    impl<Outer: ?Sized, Inner: ?Sized> Clone for ImplsTransparentWrapper<Outer, Inner> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<Outer: ?Sized, Inner: ?Sized> ImplsTransparentWrapper<Outer, Inner>
    where
        Outer: TransparentWrapper<Inner>,
    {
        /// Constructs an `ImplsTransparentWrapper`
        ///
        /// You can also use the [`infer_tw`] macro to construct `ImplsTransparentWrapper` arguments.
        pub const NEW: Self = unsafe {
            Self {
                from_inner: TransmutableInto::new_unchecked(),
                into_inner: TransmutableInto::new_unchecked(),
            }
        };
    }

    impl<Outer: ?Sized, Inner: ?Sized> ImplsTransparentWrapper<Outer, Inner> {
        const __NEW_UNCHECKED__: Self = unsafe {
            Self {
                from_inner: TransmutableInto::new_unchecked(),
                into_inner: TransmutableInto::new_unchecked(),
            }
        };

        /// Constructs an `ImplsTransparentWrapper` without checking that `T` implements
        /// [`TransparentWrapper`].
        ///
        /// # Safety
        ///
        /// You must ensure that `T` follows the
        /// [safety requirements of `TransparentWrapper`](bytemuck::TransparentWrapper#safety)
        ///
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self::__NEW_UNCHECKED__
        }
    }

    impl<Outer, Inner> ImplsTransparentWrapper<Outer, Inner> {
        /// Turns a `ImplsTransparentWrapper<Outer, Inner>` into a
        /// `ImplsTransparentWrapper<[Outer; LEN], [Inner; LEN]>`.
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{infer_tw, wrapper};
        ///
        /// #[derive(Debug, PartialEq)]
        /// #[repr(transparent)]
        /// pub struct Xyz<T>(pub T);
        ///
        /// unsafe impl<T> constmuck::TransparentWrapper<T> for Xyz<T> {}
        ///
        /// {
        ///     // Transmuting from `[u32; 5]` to `[Xyz<u32>; 5]`
        ///     const ARR: [Xyz<u32>; 5] = wrapper::wrap(
        ///         [3, 5, 13, 34, 89],
        ///         infer_tw!().array(),
        ///     );
        ///    
        ///     assert_eq!(ARR, [Xyz(3), Xyz(5), Xyz(13), Xyz(34), Xyz(89)]);
        /// }
        ///
        /// {
        ///     // Transmuting from `[Xyz<u32>; 5]` to `[u32; 5]`
        ///     const ARR: [u32; 5] = wrapper::peel(
        ///         [Xyz(3), Xyz(5), Xyz(13), Xyz(34), Xyz(89)],
        ///         infer_tw!().array::<5>(),
        ///     );
        ///    
        ///     assert_eq!(ARR, [3, 5, 13, 34, 89]);
        /// }
        /// ```
        #[inline(always)]
        pub const fn array<const LEN: usize>(
            self,
        ) -> ImplsTransparentWrapper<[Outer; LEN], [Inner; LEN]> {
            ImplsTransparentWrapper {
                from_inner: self.from_inner.array(),
                into_inner: self.into_inner.array(),
            }
        }
    }
}

impl<Outer: ?Sized, Inner: ?Sized> Infer for ImplsTransparentWrapper<Outer, Inner>
where
    Outer: TransparentWrapper<Inner>,
{
    const INFER: Self = Self::NEW;
}

////////////////////////////////////////////////////////////////////////////////

/// Trasmutes `Inner` to `Outer`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{infer_tw, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Qux<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Qux<T> {}
///
///
/// // Transmuting `&u32` to `Qux<u32>`
/// //
/// // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
/// const VALUE: Qux<u32> = wrapper::wrap(3, infer_tw!());
///
/// assert_eq!(VALUE, Qux(3));
///
/// // `infer_tw!(Qux<_>)` is required because any type can implement comparison with `Qux`.
/// assert_eq!(wrapper::wrap(3, infer_tw!(Qux<_>)), Qux(3));
///
///
/// // Transmuting `[u32; 3]` to `[Qux<u32>; 3]`
/// //
/// // The `.array()` is required to transmute arrays of values into arrays of
/// // wrappers around those values.
/// const ARR: [Qux<u32>; 3] = wrapper::wrap([5, 8, 13], infer_tw!().array());
///
/// assert_eq!(ARR, [Qux(5), Qux(8), Qux(13)]);
///
/// assert_eq!(
///     wrapper::wrap([5, 8, 13], infer_tw!(Qux<_>).array()),
///     [Qux(5), Qux(8), Qux(13)],
/// );
///
/// ```
pub const fn wrap<Inner, Outer>(val: Inner, _: ImplsTransparentWrapper<Outer, Inner>) -> Outer {
    unsafe { __priv_transmute_unchecked!(Inner, Outer, val) }
}

/// Trasmutes `&Inner` to `&Outer`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{infer_tw, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Transmuting `&u32` to `&Foo<u32>`
/// //
/// // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
/// const X: &Foo<u32> = wrapper::wrap_ref(&100, infer_tw!());
///
/// assert_eq!(X, &Foo(100));
///
/// // `infer_tw!(Foo<_>)` is required because any type can implement comparison with `Foo`.
/// assert_eq!(wrapper::wrap_ref(&100, infer_tw!(Foo<_>)), &Foo(100));
///
/// ```
pub const fn wrap_ref<Inner, Outer>(
    reff: &Inner,
    _: ImplsTransparentWrapper<Outer, Inner>,
) -> &Outer {
    unsafe {
        __priv_transmute_ref_unchecked! {Inner, Outer, reff}
    }
}

/// Trasmutes `&[Inner]` to `&[Outer]`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{infer_tw, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Bar<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Bar<T> {}
///
/// // Transmuting `&[&str]` to `&[Bar<&str>]`
/// //
/// // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
/// const X: &[Bar<&str>] = wrapper::wrap_slice(&["hello", "world"], infer_tw!());
///
/// assert_eq!(X, [Bar("hello"), Bar("world")]);
///
/// // `infer_tw!(Bar<_>)` is required because any type can implement comparison with `Bar`.
/// assert_eq!(
///     wrapper::wrap_slice(&["hello", "world"], infer_tw!(Bar<_>)),
///     [Bar("hello"), Bar("world")],
/// );
///
/// ```
pub const fn wrap_slice<Inner, Outer>(
    reff: &[Inner],
    _: ImplsTransparentWrapper<Outer, Inner>,
) -> &[Outer] {
    unsafe {
        __priv_transmute_ref_unchecked! {[Inner], [Outer], reff}
    }
}

/// Trasmutes `Outer` to `Inner`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{infer_tw, wrapper};
///
/// use std::num::Wrapping;
///
/// // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
/// const VALUE: u32 = wrapper::peel(Wrapping(3), infer_tw!());
///
///
/// // Transmuting `[Wrapping<u32>; 3]` to `[u32; 3]`
/// //
/// // The `.array()` is required to transmute arrays of wrappers into
/// // arrays of the values inside those wrappers.
/// const ARR: [u32; 3] = wrapper::peel(
///     [Wrapping(5), Wrapping(8), Wrapping(13)],
///     infer_tw!().array(),
/// );
///
/// assert_eq!(VALUE, 3);
/// assert_eq!(ARR, [5, 8, 13]);
///
/// ```
pub const fn peel<Inner, Outer>(val: Outer, _: ImplsTransparentWrapper<Outer, Inner>) -> Inner {
    unsafe { __priv_transmute_unchecked!(Outer, Inner, val) }
}

/// Trasmutes `&Outer` to `&Inner`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{infer_tw, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Transmuting `&Foo<char>` to `&char`
/// //
/// // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
/// const X: &char = wrapper::peel_ref(&Foo('@'), infer_tw!());
///
/// assert_eq!(X, &'@');
///
/// ```
pub const fn peel_ref<Inner, Outer>(
    reff: &Outer,
    _: ImplsTransparentWrapper<Outer, Inner>,
) -> &Inner {
    unsafe {
        __priv_transmute_ref_unchecked! {Outer, Inner, reff}
    }
}

/// Trasmutes `&[Outer]` to `&[Inner]`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{infer_tw, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Bar<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Bar<T> {}
///
/// // Transmuting `&[Bar<&str>]` to `&[&str]`
/// //
/// // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
/// const X: &[&str] = wrapper::peel_slice(&[Bar("hello"), Bar("world")], infer_tw!());
///
/// assert_eq!(X, ["hello", "world"]);
///
/// ```
pub const fn peel_slice<Inner, Outer>(
    reff: &[Outer],
    _: ImplsTransparentWrapper<Outer, Inner>,
) -> &[Inner] {
    unsafe {
        __priv_transmute_ref_unchecked! {[Outer], [Inner], reff}
    }
}
