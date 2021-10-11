//! Functions for wrapping/peeling types that implement [`TransparentWrapper`]
//!
//! Related: [`IsTransparentWrapper`](crate::IsTransparentWrapper) type and
//! [IsTW](crate::IsTW) macro.
//!
//! # Example
//!
//! Tranmuting between arrays of values and arrays of wrappers.
//!
//! ```rust
//! use constmuck::{IsTW, wrapper};
//!
//! #[derive(Debug, PartialEq)]
//! #[repr(transparent)]
//! pub struct Foo<T>(pub T);
//!
//! unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
//!
//! // `[u8; 3]` to `[Foo<u8>; 3]`
//! {
//!     const ARR: [Foo<u8>; 3] = wrapper::wrap([3, 5, 8], IsTW!().array());
//!
//!     assert_eq!(ARR, [Foo(3), Foo(5), Foo(8)]);
//!     // How to use `wrap` without relying on return type inference:
//!     // `IsTW!(Foo<_>)` is required because any type can implement comparison with `Foo`.
//!     assert_eq!(
//!         wrapper::wrap([13, 21, 34], IsTW!(Foo<_>).array()),
//!         [Foo(13), Foo(21), Foo(34)],
//!     );
//! }
//!
//! // `[Foo<u8>; 3]` to `[u8; 3]`
//! {
//!     const ARR: [u8; 3] = wrapper::peel([Foo(3), Foo(5), Foo(8)], IsTW!().array());
//!
//!     assert_eq!(ARR, [3, 5, 8]);
//!     assert_eq!(
//!         wrapper::peel([Foo(13), Foo(21), Foo(34)], IsTW!(Foo<_>).array()),
//!         [13, 21, 34],
//!     );
//! }
//!
//! // `&[u8; 3]` to `&[Foo<u8>; 3]`
//! {
//!     const REF_ARR: &[Foo<u8>; 3] = wrapper::wrap_ref(&[3, 5, 8], IsTW!().array());
//!
//!     assert_eq!(REF_ARR, &[Foo(3), Foo(5), Foo(8)]);
//!     assert_eq!(
//!         wrapper::wrap_ref(&[13, 21, 34], IsTW!(Foo<_>).array()),
//!         &[Foo(13), Foo(21), Foo(34)],
//!     );    
//! }
//!
//! // `&[Foo<u8>; 3]` to `&[u8; 3]`
//! {
//!     const REF_ARR: &[u8; 3] = wrapper::peel_ref(&[Foo(3), Foo(5), Foo(8)], IsTW!().array());
//!
//!     assert_eq!(REF_ARR, &[3, 5, 8]);
//!     assert_eq!(
//!         wrapper::peel_ref(&[Foo(13), Foo(21), Foo(34)], IsTW!(Foo<_>).array()),
//!         &[13, 21, 34],
//!     );    
//! }
//!
//! ```
//!

use bytemuck::TransparentWrapper;

use crate::{Infer, TransmutableInto};

#[doc(no_inline)]
pub use crate::IsTransparentWrapper;

#[doc(no_inline)]
pub use crate::IsTW;

/// Constructs an [`IsTransparentWrapper<$outer, $inner>`](crate::IsTransparentWrapper).
///
/// This has two optional type arguments (`$outer` and `$inner`) that default to
/// infering the type if not passed.
///
/// # Example
///
/// ### `constmuck::wrapper` functions
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// {
///     // Transmute `ì8` to `Foo<i8>`
///     const WRAP_VAL_ONE: Foo<i8> = wrapper::wrap(3, IsTW!());
///     
///     assert_eq!(WRAP_VAL_ONE, Foo(3));
///     assert_eq!(wrapper::wrap(3, IsTW!(Foo<i8>)), Foo(3));
///     assert_eq!(wrapper::wrap(3, IsTW!(Foo<i8>, i8)), Foo(3));
///     
///     
///     // Transmute `[u8; 3]` to `[Foo<u8>; 3]`
///     //
///     // The `.array()` is required to cast arrays of values into arrays of
///     // wrappers around those values.
///     const WRAP_VAL_ARR: [Foo<u8>; 3] = wrapper::wrap([5, 8, 13], IsTW!().array());
///     
///     assert_eq!(WRAP_VAL_ARR, [Foo(5), Foo(8), Foo(13)]);
///     assert_eq!(
///         wrapper::wrap([5, 8, 13], IsTW!(Foo<u8>).array()),
///         [Foo(5), Foo(8), Foo(13)],
///     );
///     assert_eq!(
///         wrapper::wrap([5, 8, 13], IsTW!(Foo<u8>, u8).array()),
///         [Foo(5), Foo(8), Foo(13)],
///     );
/// }
/// {
///     // Transmute `&i8` to `&Foo<i8>`
///     const WRAP_REF_ONE: &Foo<i8> = wrapper::wrap_ref(&3, IsTW!());
///     
///     // Transmute `&[u8; 3]` to `&[Foo<u8>; 3]`
///     const WRAP_REF_ARR: &[Foo<u8>; 3] = wrapper::wrap_ref(&[5, 8, 13], IsTW!().array());
///     
///     assert_eq!(WRAP_REF_ONE, &Foo(3));
///     assert_eq!(WRAP_REF_ARR, &[Foo(5), Foo(8), Foo(13)]);
/// }
/// {
///     // Transmute `&[i8]` to `&[Foo<i8>]`
///     const WRAP_SLICE: &[Foo<i8>] = wrapper::wrap_slice(&[21, 34, 55], IsTW!());
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
///     IsTW,
/// };
///
/// use std::num::Wrapping;
///
/// {
///     // Casting `&Wrapping<u8>` to `&u8`,
///     //
///     // `IsTW!()` constructs an `IsTransparentWrapper`,
///     // whose `into_inner` field allows casting from a wrapper into the value in it.
///     const UNWRAPPED: &u8 = transmute_ref(&Wrapping(5), IsTW!().into_inner);
///     assert_eq!(*UNWRAPPED, 5);
/// }
///
/// {
///     // Casting `&u8` to `&Wrapping<u8>`
///     //
///     // `IsTW!()` constructs an `IsTransparentWrapper`,
///     // whose `from_inner` field allows casting from a value into a wrapper around it.
///     const WRAPPED: &Wrapping<u8> = transmute_ref(&7, IsTW!().from_inner);
///    
///     assert_eq!(*WRAPPED, Wrapping(7));
/// }
///
/// ```
#[macro_export]
macro_rules! IsTW {
    () => {
        <$crate::IsTransparentWrapper<_, _> as $crate::Infer>::INFER
    };
    ($outer:ty $(,)*) => {
        <$crate::IsTransparentWrapper<$outer, _> as $crate::Infer>::INFER
    };
    ($outer:ty, $inner:ty $(,)*) => {
        <$crate::IsTransparentWrapper<$outer, $inner> as $crate::Infer>::INFER
    };
}

pub(crate) mod is_tw {
    use super::*;

    /// Encodes a `Outer:`[`TransparentWrapper`]`<Inner>` bound as a value.
    ///
    /// Constructible with [`NEW`](Self::NEW) associated constant,
    /// or [`IsTW`](macro@crate::IsTW) macro.
    ///
    /// Related: [`wrapper`](crate::wrapper) module.
    ///
    /// # Example
    ///
    /// ```rust
    /// use constmuck::{IsTW, wrapper};
    ///
    /// #[derive(Debug, PartialEq)]
    /// #[repr(transparent)]
    /// pub struct This<T>(pub T);
    ///
    /// unsafe impl<T> constmuck::TransparentWrapper<T> for This<T> {}
    ///
    /// {
    ///     // Casting from `&&str` to `&This<&str>`
    ///     //
    ///     // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
    ///     const WRAPPED: &This<&str> = wrapper::wrap_ref(&"hi", IsTW!());
    ///     assert_eq!(*WRAPPED, This("hi"));
    /// }
    ///
    /// {
    ///     // Casting from `&This<&str>` to `&&str`
    ///     const UNWRAPPED: &&str = wrapper::peel_ref(&This("hello"), IsTW!());
    ///     assert_eq!(*UNWRAPPED, "hello");
    /// }
    ///
    /// {
    ///     // Casting from `[u64; 2]` to `[This<u64>; 2]`
    ///     //
    ///     // The `.array()` is required to cast arrays of values into arrays of
    ///     // wrappers around those values.
    ///     const WRAPPED_ARR: [This<u64>; 2] = wrapper::wrap([9, 99], IsTW!().array());
    ///     assert_eq!(WRAPPED_ARR, [This(9), This(99)]);
    /// }
    ///
    /// {
    ///     // Casting from `[This<i8>; 2]` to `[i8; 2]`
    ///     //
    ///     // `.array()` also allows casting arrays of wrappers into
    ///     // arrays of the values inside those wrappers, using the `peel*` functions.
    ///     const UNWRAPPED_ARR: [i8; 2] =
    ///         wrapper::peel([This(2), This(22)], IsTW!().array());
    ///     assert_eq!(UNWRAPPED_ARR, [2, 22]);
    /// }
    ///
    /// ```
    pub struct IsTransparentWrapper<Outer: ?Sized, Inner: ?Sized> {
        pub from_inner: TransmutableInto<Inner, Outer>,
        pub into_inner: TransmutableInto<Outer, Inner>,
        #[doc(hidden)]
        pub _transparent_wrapper_proof: constmuck_internal::TransparentWrapperProof<Outer, Inner>,
    }

    impl<Outer: ?Sized, Inner: ?Sized> Copy for IsTransparentWrapper<Outer, Inner> {}

    impl<Outer: ?Sized, Inner: ?Sized> Clone for IsTransparentWrapper<Outer, Inner> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<Outer: ?Sized, Inner: ?Sized> IsTransparentWrapper<Outer, Inner>
    where
        Outer: TransparentWrapper<Inner>,
    {
        /// Constructs an `IsTransparentWrapper`
        ///
        /// You can also use the [`IsTW`](macro@crate::IsTW) macro to
        /// construct `IsTransparentWrapper` arguments.
        pub const NEW: Self = unsafe {
            Self {
                from_inner: TransmutableInto::new_unchecked(),
                into_inner: TransmutableInto::new_unchecked(),
                _transparent_wrapper_proof:
                    constmuck_internal::TransparentWrapperProof::new_unchecked(),
            }
        };
    }

    impl<Outer: ?Sized, Inner: ?Sized> IsTransparentWrapper<Outer, Inner> {
        const __NEW_UNCHECKED__: Self = unsafe {
            Self {
                from_inner: TransmutableInto::new_unchecked(),
                into_inner: TransmutableInto::new_unchecked(),
                _transparent_wrapper_proof:
                    constmuck_internal::TransparentWrapperProof::new_unchecked(),
            }
        };

        /// Constructs an `IsTransparentWrapper` without checking that
        /// `Outer` implements
        /// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper).
        ///
        /// # Safety
        ///
        /// You must ensure that `Outer` follows the
        /// [safety requirements of `TransparentWrapper<Inner>`
        /// ](bytemuck::TransparentWrapper#safety)
        ///
        #[inline(always)]
        pub const unsafe fn new_unchecked() -> Self {
            Self::__NEW_UNCHECKED__
        }

        /// Constructs an `IsTransparentWrapper` from a pair
        /// of [`TransmutableInto`] that allow transmuting between `Outer` and `Inner`
        /// in both directions.
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{IsTransparentWrapper, TransmutableInto, infer, wrapper};
        ///
        /// const ITW: IsTransparentWrapper<u8, i8> =
        ///     IsTransparentWrapper::from_ti(
        ///         TransmutableInto::pod(infer!()),
        ///         TransmutableInto::pod(infer!()),
        ///     );
        ///
        /// const U8_VAL: u8 = wrapper::wrap(-1, ITW);
        /// assert_eq!(U8_VAL, 255);
        ///
        /// const U8_REF: &u8 = wrapper::wrap_ref(&-2, ITW);
        /// assert_eq!(U8_REF, &254);
        ///
        /// const U8_SLICE: &[u8] = wrapper::wrap_slice(&[-3, 3], ITW);
        /// assert_eq!(U8_SLICE, &[253, 3]);
        ///
        /// const I8_VAL: i8 = wrapper::peel(128, ITW);
        /// assert_eq!(I8_VAL, -128);
        ///
        /// const I8_REF: &i8 = wrapper::peel_ref(&129, ITW);
        /// assert_eq!(I8_REF, &-127);
        ///
        /// const I8_SLICE: &[i8] = wrapper::peel_slice(&[254, 1], ITW);
        /// assert_eq!(I8_SLICE, &[-2, 1]);
        ///
        ///
        ///
        /// ```
        pub const fn from_ti(
            from_inner: TransmutableInto<Inner, Outer>,
            into_inner: TransmutableInto<Outer, Inner>,
        ) -> Self {
            Self {
                from_inner,
                into_inner,
                _transparent_wrapper_proof: unsafe {
                    constmuck_internal::TransparentWrapperProof::new_unchecked()
                },
            }
        }
    }

    impl<Outer: ?Sized, Inner: ?Sized> IsTransparentWrapper<Outer, Inner> {
        /// Combines an `IsTransparentWrapper` with another to allow
        /// casting between `Outer` and `Nested`.
        ///
        /// Without this you'd have to do `Outer -> Inner -> Nested` casts.
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{IsTW, IsTransparentWrapper, wrapper};
        ///
        /// use std::num::Wrapping;
        ///
        /// const FOO: IsTransparentWrapper<Bar<u32>, u32> = IsTW!().join(IsTW!());
        ///
        /// // Equivalent to FOO, but passing the types to `IsTW`.
        /// // Only the innermost joined IsTransparentWrapper requires you
        /// // to pass both arguments to `IsTW`.
        /// let foo = IsTW!(Bar<u32>).join(IsTW!(Wrapping<u32>, u32));
        ///
        /// assert_eq!(wrapper::wrap_ref(&5, FOO), &Bar(Wrapping(5)));
        /// assert_eq!(wrapper::wrap_ref(&8, foo), &Bar(Wrapping(8)));
        ///
        /// assert_eq!(
        ///     wrapper::wrap_slice(&[13, 21], FOO),
        ///     &[Bar(Wrapping(13)), Bar(Wrapping(21))]
        /// );
        ///
        ///
        /// #[derive(Debug, PartialEq)]
        /// struct Bar<T>(Wrapping<T>);
        ///
        /// unsafe impl<T> constmuck::TransparentWrapper<Wrapping<T>> for Bar<T> {}
        ///
        /// ```
        pub const fn join<Nested: ?Sized>(
            self,
            _other: IsTransparentWrapper<Inner, Nested>,
        ) -> IsTransparentWrapper<Outer, Nested> {
            IsTransparentWrapper::__NEW_UNCHECKED__
        }
    }

    impl<Outer, Inner> IsTransparentWrapper<Outer, Inner> {
        /// Turns a `IsTransparentWrapper<Outer, Inner>` into a
        /// `IsTransparentWrapper<[Outer; LEN], [Inner; LEN]>`.
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{IsTW, wrapper};
        ///
        /// #[derive(Debug, PartialEq)]
        /// #[repr(transparent)]
        /// pub struct Xyz<T>(pub T);
        ///
        /// unsafe impl<T> constmuck::TransparentWrapper<T> for Xyz<T> {}
        ///
        /// {
        ///     // Casting from `[u32; 5]` to `[Xyz<u32>; 5]`
        ///     const ARR: [Xyz<u32>; 5] = wrapper::wrap(
        ///         [3, 5, 13, 34, 89],
        ///         IsTW!().array(),
        ///     );
        ///    
        ///     assert_eq!(ARR, [Xyz(3), Xyz(5), Xyz(13), Xyz(34), Xyz(89)]);
        /// }
        ///
        /// {
        ///     // Casting from `[Xyz<u32>; 5]` to `[u32; 5]`
        ///     const ARR: [u32; 5] = wrapper::peel(
        ///         [Xyz(3), Xyz(5), Xyz(13), Xyz(34), Xyz(89)],
        ///         IsTW!().array::<5>(),
        ///     );
        ///    
        ///     assert_eq!(ARR, [3, 5, 13, 34, 89]);
        /// }
        /// ```
        #[inline(always)]
        pub const fn array<const LEN: usize>(
            self,
        ) -> IsTransparentWrapper<[Outer; LEN], [Inner; LEN]> {
            IsTransparentWrapper {
                from_inner: self.from_inner.array(),
                into_inner: self.into_inner.array(),
                _transparent_wrapper_proof: unsafe {
                    constmuck_internal::TransparentWrapperProof::new_unchecked()
                },
            }
        }
    }
}

impl<Outer: ?Sized, Inner: ?Sized> Infer for IsTransparentWrapper<Outer, Inner>
where
    Outer: TransparentWrapper<Inner>,
{
    const INFER: Self = Self::NEW;
}

////////////////////////////////////////////////////////////////////////////////

/// Casts `Inner` to `Outer`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Qux<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Qux<T> {}
///
///
/// // Casting `&u32` to `Qux<u32>`
/// //
/// // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
/// const VALUE: Qux<u32> = wrapper::wrap(3, IsTW!());
///
/// assert_eq!(VALUE, Qux(3));
///
/// // `IsTW!(Qux<_>)` is required because any type can implement comparison with `Qux`.
/// assert_eq!(wrapper::wrap(3, IsTW!(Qux<_>)), Qux(3));
///
///
/// // Casting `[u32; 3]` to `[Qux<u32>; 3]`
/// //
/// // The `.array()` is required to cast arrays of values into arrays of
/// // wrappers around those values.
/// const ARR: [Qux<u32>; 3] = wrapper::wrap([5, 8, 13], IsTW!().array());
///
/// assert_eq!(ARR, [Qux(5), Qux(8), Qux(13)]);
///
/// assert_eq!(
///     wrapper::wrap([5, 8, 13], IsTW!(Qux<_>).array()),
///     [Qux(5), Qux(8), Qux(13)],
/// );
///
/// ```
pub const fn wrap<Inner, Outer>(val: Inner, _: IsTransparentWrapper<Outer, Inner>) -> Outer {
    __check_same_alignment! {Outer, Inner}

    unsafe { __priv_transmute!(Inner, Outer, val) }
}

/// Casts `&Inner` to `&Outer`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// To cast references to `?Sized` types, you need to use the
/// [`wrap_ref`](macro@crate::wrapper::wrap_ref) macro instead of this function.
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Casting `&u32` to `&Foo<u32>`
/// //
/// // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
/// const X: &Foo<u32> = wrapper::wrap_ref(&100, IsTW!());
///
/// assert_eq!(X, &Foo(100));
///
/// // `IsTW!(Foo<_>)` is required because any type can implement comparison with `Foo`.
/// assert_eq!(wrapper::wrap_ref(&100, IsTW!(Foo<_>)), &Foo(100));
///
/// ```
pub const fn wrap_ref<Inner, Outer>(reff: &Inner, _: IsTransparentWrapper<Outer, Inner>) -> &Outer {
    __check_same_alignment! {Outer, Inner}

    unsafe {
        __priv_transmute_ref! {Inner, Outer, reff}
    }
}

/// Casts `&Inner` to `&Outer`, allows casting between `?Sized` types.
///
/// This is equivalent to a function with this signature:
///
/// ```rust
/// pub const fn wrap_ref<Inner: ?Sized, Outer: ?Sized>(
///     reff: &Inner,
///     is_tw: constmuck::IsTransparentWrapper<Outer, Inner>
/// ) -> &Outer
/// # { loop{} }
/// ```
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// Note that, because of how this macro is implemented,
/// [`infer`] cannot be passed as the `is_tw` argument.
/// You must pass a type that's known to be an
/// [`IsTransparentWrapper`](crate::IsTransparentWrapper) beforehand,
/// eg: [`IsTW!()`](macro@crate::IsTW), [`IsTransparentWrapper::NEW`].
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T: ?Sized>(pub T);
///
/// unsafe impl<T: ?Sized> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Casting `&str` to `&Foo<str>`
/// //
/// // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
/// const X: &Foo<str> = wrapper::wrap_ref!("world", IsTW!());
///
/// assert_eq!(X.0, *"world");
///
/// // `IsTW!(Foo<_>)` is required because any type can implement comparison with `Foo`.
/// assert_eq!(wrapper::wrap_ref!("huh", IsTW!(Foo<_>)).0, *"huh");
///
/// ```
pub use constmuck_internal::wrapper_wrap_ref as wrap_ref;

/// Casts `&[Inner]` to `&[Outer]`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Bar<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Bar<T> {}
///
/// // Casting `&[&str]` to `&[Bar<&str>]`
/// //
/// // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
/// const X: &[Bar<&str>] = wrapper::wrap_slice(&["hello", "world"], IsTW!());
///
/// assert_eq!(X, [Bar("hello"), Bar("world")]);
///
/// // `IsTW!(Bar<_>)` is required because any type can implement comparison with `Bar`.
/// assert_eq!(
///     wrapper::wrap_slice(&["hello", "world"], IsTW!(Bar<_>)),
///     [Bar("hello"), Bar("world")],
/// );
///
/// ```
pub const fn wrap_slice<Inner, Outer>(
    reff: &[Inner],
    _: IsTransparentWrapper<Outer, Inner>,
) -> &[Outer] {
    __check_same_alignment! {Outer, Inner}

    unsafe {
        __priv_transmute_slice! {Inner, Outer, reff}
    }
}

/// Casts `Outer` to `Inner`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// use std::num::Wrapping;
///
/// // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
/// const VALUE: u32 = wrapper::peel(Wrapping(3), IsTW!());
///
///
/// // Casting `[Wrapping<u32>; 3]` to `[u32; 3]`
/// //
/// // The `.array()` is required to cast arrays of wrappers into
/// // arrays of the values inside those wrappers.
/// const ARR: [u32; 3] = wrapper::peel(
///     [Wrapping(5), Wrapping(8), Wrapping(13)],
///     IsTW!().array(),
/// );
///
/// assert_eq!(VALUE, 3);
/// assert_eq!(ARR, [5, 8, 13]);
///
/// ```
pub const fn peel<Inner, Outer>(val: Outer, _: IsTransparentWrapper<Outer, Inner>) -> Inner {
    __check_same_alignment! {Outer, Inner}

    unsafe { __priv_transmute!(Outer, Inner, val) }
}

/// Casts `&Outer` to `&Inner`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// To cast references to `?Sized` types, you need to use the
/// [`peel_ref`](macro@crate::wrapper::peel_ref) macro instead of this function.
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Casting `&Foo<char>` to `&char`
/// //
/// // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
/// const X: &char = wrapper::peel_ref(&Foo('@'), IsTW!());
///
/// assert_eq!(X, &'@');
///
/// ```
pub const fn peel_ref<Inner, Outer>(reff: &Outer, _: IsTransparentWrapper<Outer, Inner>) -> &Inner {
    __check_same_alignment! {Outer, Inner}

    unsafe {
        __priv_transmute_ref! {Outer, Inner, reff}
    }
}

/// Casts `&Outer` to `&Inner`, allows casting between `?Sized` types
///
/// This is equivalent to a function with this signature:
///
/// ```rust
/// pub const fn peel_ref<Inner: ?Sized, Outer: ?Sized>(
///     reff: &Outer,
///     is_tw: constmuck::IsTransparentWrapper<Outer, Inner>
/// ) -> &Inner
/// # { loop{} }
/// ```
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// Note that, because of how this macro is implemented,
/// [`infer`] cannot be passed as the `is_tw` argument.
/// You must pass a type that's known to be an
/// [`IsTransparentWrapper`](crate::IsTransparentWrapper) beforehand,
/// eg: [`IsTW!()`](macro@crate::IsTW), [`IsTransparentWrapper::NEW`].
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Foo<T: ?Sized>(pub T);
///
/// unsafe impl<T: ?Sized> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// const X: &[u8] = {
///     let x: &'static Foo<[u8]> = &Foo([3, 5, 8, 13]);
///
///     // Casting `&Foo<[u8]>` to `&[u8]`
///     //
///     // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
///     wrapper::peel_ref!(x, IsTW!())
/// };
///
/// assert_eq!(X, [3, 5, 8, 13]);
///
/// ```
pub use constmuck_internal::wrapper_peel_ref as peel_ref;

/// Casts `&[Outer]` to `&[Inner]`
///
/// Requires that `Outer` implements
/// [`TransparentWrapper<Inner>`](bytemuck::TransparentWrapper)
///
/// # Example
///
/// ```rust
/// use constmuck::{IsTW, wrapper};
///
/// #[derive(Debug, PartialEq)]
/// #[repr(transparent)]
/// pub struct Bar<T>(pub T);
///
/// unsafe impl<T> constmuck::TransparentWrapper<T> for Bar<T> {}
///
/// // Casting `&[Bar<&str>]` to `&[&str]`
/// //
/// // `IsTW!()` is a more concise way to write `IsTransparentWrapper::NEW`
/// const X: &[&str] = wrapper::peel_slice(&[Bar("hello"), Bar("world")], IsTW!());
///
/// assert_eq!(X, ["hello", "world"]);
///
/// ```
pub const fn peel_slice<Inner, Outer>(
    reff: &[Outer],
    _: IsTransparentWrapper<Outer, Inner>,
) -> &[Inner] {
    __check_same_alignment! {Outer, Inner}

    unsafe {
        __priv_transmute_slice! {Outer, Inner, reff}
    }
}
