//! Functions for wrapping/peeling types that implement
//! [`TransparentWrapper`](trait@TransparentWrapper).
//!
use bytemuck::TransparentWrapper;

////////////////////////////////////////////////////////////////////////////////

/// Casts `Inner` to `Outer`
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// # #[derive(Debug, PartialEq)]
/// # /*
/// #[derive(Debug, PartialEq, constmuck::TransparentWrapper)]
/// # */
/// #[repr(transparent)]
/// pub struct Qux<T>(pub T);
/// #
/// # unsafe impl<T> constmuck::TransparentWrapper<T> for Qux<T> {}
///
///
/// // Casting `u32` to `Qux<u32>`
/// const VALUE: Qux<u32> = wrapper::wrap(3);
///
/// assert_eq!(VALUE, Qux(3));
///
/// // the `::<Qux<_>, _>` is required because any type can implement comparison with `Qux`.
/// assert_eq!(wrapper::wrap::<Qux<_>, _>(3), Qux(3));
///
/// ```
pub const fn wrap<Outer, Inner>(val: Inner) -> Outer
where
    Outer: TransparentWrapper<Inner>,
{
    __check_same_alignment! {Outer, Inner}

    // safety: `Outer: TransparentWrapper<Inner>` guarantees that
    //         `Outer` has the same layout as `Inner`
    unsafe { __priv_transmute!(Inner, Outer, val) }
}

/// Casts `&Inner` to `&Outer`
///
/// To cast references to `!Sized` types, you need to use the
/// [`wrap_ref`](macro@crate::wrapper::wrap_ref) macro instead of this function.
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// # #[derive(Debug, PartialEq)]
/// # /*
/// #[derive(Debug, PartialEq, constmuck::TransparentWrapper)]
/// # */
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
/// #
/// # unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Casting `&u32` to `&Foo<u32>`
/// const X: &Foo<u32> = wrapper::wrap_ref(&100);
///
/// assert_eq!(X, &Foo(100));
///
/// // `::<Foo<_>, _>` is required because any type can implement comparison with `Foo`.
/// assert_eq!(wrapper::wrap_ref::<Foo<_>, _>(&100), &Foo(100));
///
/// ```
pub const fn wrap_ref<Outer, Inner>(reff: &Inner) -> &Outer
where
    Outer: TransparentWrapper<Inner>,
{
    __check_same_alignment! {Outer, Inner}

    // safety: `Outer: TransparentWrapper<Inner>` guarantees that
    //         `Outer` has the same layout as `Inner`
    unsafe {
        __priv_transmute_ref! {Inner, Outer, reff}
    }
}

/// Casts `&Inner` to `&Outer`, allows casting between `?Sized` types.
///
/// This macro is equivalent to a function with this signature:
///
/// ```rust
/// # use bytemuck::TransparentWrapper;
/// pub const fn wrap_ref<Outer: ?Sized, Inner: ?Sized>(
///     reff: &Inner,
/// ) -> &Outer
/// where
///     Outer: TransparentWrapper<Inner>
/// # { loop{} }
/// ```
///
/// The optional `$Outer:ty` and `$Inner:ty` parameters correspond to the
/// `Outer` and `Inner` type parameters, and are inferred if not passed.
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// # #[derive(Debug, PartialEq)]
/// # /*
/// #[derive(Debug, PartialEq, constmuck::TransparentWrapper)]
/// # */
/// #[repr(transparent)]
/// pub struct Foo<T: ?Sized>(pub T);
/// #
/// # unsafe impl<T: ?Sized> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Casting `&str` to `&Foo<str>`
/// const X: &Foo<str> = wrapper::wrap_ref!("world");
///
/// assert_eq!(X.0, *"world");
///
/// // specifying what type the macro evaluates into
/// let foo = wrapper::wrap_ref!("huh", Foo<_>);
/// assert_eq!(foo.0, *"huh");
///
/// ```
pub use constmuck_internal::wrapper_wrap_ref as wrap_ref;

/// Casts `&[Inner]` to `&[Outer]`
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// # #[derive(Debug, PartialEq)]
/// # /*
/// #[derive(Debug, PartialEq, constmuck::TransparentWrapper)]
/// # */
/// #[repr(transparent)]
/// pub struct Bar<T>(pub T);
/// #
/// # unsafe impl<T> constmuck::TransparentWrapper<T> for Bar<T> {}
///
/// // Casting `&[&str]` to `&[Bar<&str>]`
/// const X: &[Bar<&str>] = wrapper::wrap_slice(&["hello", "world"]);
///
/// assert_eq!(X, [Bar("hello"), Bar("world")]);
///
/// // `::<Bar<_>, _>` is required because `Bar` could be comparable with many types.
/// assert_eq!(
///     wrapper::wrap_slice::<Bar<_>, _>(&["hello", "world"]),
///     [Bar("hello"), Bar("world")],
/// );
///
/// ```
pub const fn wrap_slice<Outer, Inner>(reff: &[Inner]) -> &[Outer]
where
    Outer: TransparentWrapper<Inner>,
{
    __check_same_alignment! {Outer, Inner}

    // safety: `Outer: TransparentWrapper<Inner>` guarantees that
    //         `Outer` has the same layout as `Inner`
    unsafe {
        __priv_transmute_slice! {Inner, Outer, reff}
    }
}

/// Casts `Outer` to `Inner`
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// use std::num::Wrapping;
///
/// const VALUE: u32 = wrapper::peel(Wrapping(3));
/// assert_eq!(VALUE, 3);
///
/// ```
pub const fn peel<Outer, Inner>(val: Outer) -> Inner
where
    Outer: TransparentWrapper<Inner>,
{
    __check_same_alignment! {Outer, Inner}

    // safety: `Outer: TransparentWrapper<Inner>` guarantees that
    //         `Outer` has the same layout as `Inner`
    unsafe { __priv_transmute!(Outer, Inner, val) }
}

/// Casts `&Outer` to `&Inner`
///
/// To cast references to `!Sized` types, you need to use the
/// [`peel_ref`](macro@crate::wrapper::peel_ref) macro instead of this function.
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// # #[derive(Debug, PartialEq)]
/// # /*
/// #[derive(Debug, PartialEq, constmuck::TransparentWrapper)]
/// # */
/// #[repr(transparent)]
/// pub struct Foo<T>(pub T);
/// #
/// # unsafe impl<T> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// // Casting `&Foo<char>` to `&char`
/// const X: &char = wrapper::peel_ref(&Foo('@'));
///
/// assert_eq!(X, &'@');
///
/// ```
pub const fn peel_ref<Outer, Inner>(reff: &Outer) -> &Inner
where
    Outer: TransparentWrapper<Inner>,
{
    __check_same_alignment! {Outer, Inner}

    // safety: `Outer: TransparentWrapper<Inner>` guarantees that
    //         `Outer` has the same layout as `Inner`
    unsafe {
        __priv_transmute_ref! {Outer, Inner, reff}
    }
}

/// Casts `&Outer` to `&Inner`, allows casting between `?Sized` types
///
/// This macro is equivalent to a function with this signature:
///
/// ```rust
/// # use bytemuck::TransparentWrapper;
/// pub const fn peel_ref<Outer: ?Sized, Inner: ?Sized>(
///     reff: &Outer,
/// ) -> &Inner
/// where
///     Outer: TransparentWrapper<Inner>
/// # { loop{} }
/// ```
///
/// The optional `$Outer:ty` and `$Inner:ty` parameters correspond to the
/// `Outer` and `Inner` type parameters, and are inferred if not passed.
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// # #[derive(Debug, PartialEq)]
/// # /*
/// #[derive(Debug, PartialEq, constmuck::TransparentWrapper)]
/// # */
/// #[repr(transparent)]
/// pub struct Foo<T: ?Sized>(pub T);
/// #
/// # unsafe impl<T: ?Sized> constmuck::TransparentWrapper<T> for Foo<T> {}
///
/// const X: &[u8] = {
///     let x: &'static Foo<[u8]> = &Foo([3, 5, 8, 13]);
///
///     // Casting `&Foo<[u8]>` to `&[u8]`
///     wrapper::peel_ref!(x)
/// };
///
/// assert_eq!(X, [3, 5, 8, 13]);
///
/// ```
pub use constmuck_internal::wrapper_peel_ref as peel_ref;

/// Casts `&[Outer]` to `&[Inner]`
///
/// # Example
///
/// ```rust
/// use constmuck::wrapper;
///
/// # #[derive(Debug, PartialEq)]
/// # /*
/// #[derive(Debug, PartialEq, constmuck::TransparentWrapper)]
/// # */
/// #[repr(transparent)]
/// pub struct Bar<T>(pub T);
/// #
/// # unsafe impl<T> constmuck::TransparentWrapper<T> for Bar<T> {}
///
/// // Casting `&[Bar<&str>]` to `&[&str]`
/// const X: &[&str] = wrapper::peel_slice(&[Bar("hello"), Bar("world")]);
///
/// assert_eq!(X, ["hello", "world"]);
///
/// ```
pub const fn peel_slice<Outer, Inner>(reff: &[Outer]) -> &[Inner]
where
    Outer: TransparentWrapper<Inner>,
{
    __check_same_alignment! {Outer, Inner}

    // safety: `Outer: TransparentWrapper<Inner>` guarantees that
    //         `Outer` has the same layout as `Inner`
    unsafe {
        __priv_transmute_slice! {Outer, Inner, reff}
    }
}
