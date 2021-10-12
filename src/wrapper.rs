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

use core::fmt::{self, Debug};

use crate::Infer;

#[doc(no_inline)]
pub use crate::IsTransparentWrapper;

#[doc(no_inline)]
pub use crate::IsTW;

/// Constructs an [`IsTransparentWrapper<$Outer, $Inner>`](crate::IsTransparentWrapper),
/// requiring that `$Outer` implements [`TransparentWrapper`]`<$Inner>`.
///
/// This has two optional type arguments (`$Outer` and `$Inner`) that default to
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
#[macro_export]
macro_rules! IsTW {
    () => {
        <$crate::IsTransparentWrapper<_, _> as $crate::Infer>::INFER
    };
    ($Outer:ty $(,)*) => {
        <$crate::IsTransparentWrapper<$Outer, _> as $crate::Infer>::INFER
    };
    ($Outer:ty, $Inner:ty $(,)*) => {
        <$crate::IsTransparentWrapper<$Outer, $Inner> as $crate::Infer>::INFER
    };
}

pub(crate) mod is_tw {
    use super::*;

    /// Encodes a `Outer:`[`TransparentWrapper`]`<Inner>` bound as a value.
    ///
    /// Related: [`IsTW`](macro@crate::IsTW) macro, [`wrapper`](crate::wrapper) module.
    ///
    /// You can also [construct an `IsTransparentWrapper<T, T>`](Self::IDENTITY),
    /// which allows using a type wherever transparent wrappers to it are expected.
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
    #[non_exhaustive]
    pub struct IsTransparentWrapper<Outer: ?Sized, Inner: ?Sized> {
        #[doc(hidden)]
        pub _transparent_wrapper_proof: constmuck_internal::TransparentWrapperProof<Outer, Inner>,
    }

    impl<Outer: ?Sized, Inner: ?Sized> Debug for IsTransparentWrapper<Outer, Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("IsTransparentWrapper")
        }
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
                _transparent_wrapper_proof:
                    constmuck_internal::TransparentWrapperProof::new_unchecked(),
            }
        };
    }

    impl<T: ?Sized> IsTransparentWrapper<T, T> {
        /// Constructs an `IsTransparentWrapper<T, T>`.
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{IsTransparentWrapper as IsTW, wrapper};
        ///
        /// use std::num::Wrapping;
        ///
        /// const fn add_u32<W>(left: u32, right: &W, is_u32: IsTW<W, u32>) -> u32 {
        ///     left + *wrapper::peel_ref(right, is_u32)
        /// }
        ///
        /// assert_eq!(add_u32(3, &Wrapping(5), IsTW::NEW), 8);
        ///
        /// assert_eq!(add_u32(8, &13, IsTW::IDENTITY), 21);
        ///
        /// ```
        pub const IDENTITY: Self = unsafe {
            Self {
                _transparent_wrapper_proof:
                    constmuck_internal::TransparentWrapperProof::new_unchecked(),
            }
        };
    }

    impl<Outer: ?Sized, Inner: ?Sized> IsTransparentWrapper<Outer, Inner> {
        const __NEW_UNCHECKED__: Self = unsafe {
            Self {
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
