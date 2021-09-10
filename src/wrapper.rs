//! Functions for wrapping/peeling types that implement [`TransparentWrapper`]
//!
//! Related: [`ImplsTransparentWrapper`] type and [`infer_tw`] macro.
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
///     // The `.array()` is required to wrap arrays of types that impl `TransparentWrapper`.
///     const WRAP_VAL_ARR: [Foo<u8>; 3] = wrapper::wrap([5, 8, 13], infer_tw!().array());
///     
///     assert_eq!(WRAP_VAL_ONE, Foo(3));
///     assert_eq!(WRAP_VAL_ARR, [Foo(5), Foo(8), Foo(13)]);
/// }
/// {
///     // Transmute `&i8` to `&Foo<i8>`
///     const WRAP_REF_ONE: &Foo<i8> = wrapper::wrap_ref(&3, infer_tw!());
///     
///     // Transmute `&[u8; 3]` to `&[Foo<u8>; 3]`
///     // The `.array()` is required to wrap arrays of types that impl `TransparentWrapper`.
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
///     // `infer_tw!().into_inner` allows transmuting
///     // from `T` to `U` where `T` impls `TransparentWrapper<U>`.
///     const UNWRAPPED: &u8 = transmute_ref(&Wrapping(5), infer_tw!().into_inner);
///     assert_eq!(*UNWRAPPED, 5);
/// }
///
/// {
///     // Transmuting `&u8` to `&Wrapping<u8>`
///     //
///     // `infer_tw!().from_inner` allows transmuting
///     // from `U` to `T` where `T` impls `TransparentWrapper<U>`.
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
    ($outer:ty, $inner:ty $(,)*) => {
        <$crate::ImplsTransparentWrapper<$outer, $inner> as $crate::Infer>::INFER
    };
}

pub(crate) mod impls_tw {
    use super::*;

    /// Encodes a `T: TransparentWrapper` bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// Related: [`wrapper`](crate::wrapper) module
    ///
    pub struct ImplsTransparentWrapper<Outer, Inner> {
        pub from_inner: TransmutableInto<Inner, Outer>,
        pub into_inner: TransmutableInto<Outer, Inner>,
    }

    impl<Outer, Inner> Copy for ImplsTransparentWrapper<Outer, Inner> {}

    impl<Outer, Inner> Clone for ImplsTransparentWrapper<Outer, Inner> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<Outer, Inner> ImplsTransparentWrapper<Outer, Inner>
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

    impl<Outer, Inner> ImplsTransparentWrapper<Outer, Inner> {
        /// Turns a `ImplsTransparentWrapper<Outer, Inner>` into a
        /// `ImplsTransparentWrapper<[Outer; LEN], [Inner; LEN]>`.
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

impl<Outer, Inner> Infer for ImplsTransparentWrapper<Outer, Inner>
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
/// use std::num::Wrapping;
///
/// // Transmuting `&u32` to `Wrapping<u32>`
/// //
/// // `infer_tw!()` is a more concise way to write `ImplsTransparentWrapper::NEW`
/// const VALUE: Wrapping<u32> = wrapper::wrap(3, infer_tw!());
///
/// // Transmuting `[u32; 3]` to `[Wrapping<u32>; 3]`
/// //
/// // The `.array()` is required to wrap arrays of types that impl `TransparentWrapper`.
/// const ARR: [Wrapping<u32>; 3] = wrapper::wrap([5, 8, 13], infer_tw!().array());
///
/// assert_eq!(VALUE, Wrapping(3));
/// assert_eq!(ARR, [Wrapping(5), Wrapping(8), Wrapping(13)]);
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
/// // The `.array()` is required to unwrap arrays of types that impl `TransparentWrapper`.
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
/// use std::num::Wrapping;
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
/// use std::num::Wrapping;
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
