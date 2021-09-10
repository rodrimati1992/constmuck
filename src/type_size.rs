use core::{
    marker::PhantomData,
    mem::{self, ManuallyDrop},
};

use crate::Infer;

/// Constructs a [`TypeSize`].
///
/// This implicitly constructs the `bounds` field of a `TypeSize` with [`Infer::INFER`].
///
/// # Example
///
/// Making a `oned` function
///
/// ```rust
/// use constmuck::{ImplsPod, TypeSize};
/// use constmuck::{infer, type_size};
///
/// pub const fn oned<T, const SIZE: usize>(bound: TypeSize<ImplsPod<T>, T, SIZE>) -> T {
///     constmuck::cast::<[u8; SIZE], T>(
///         [1; SIZE],
///         // `infer!()` here constructs an `ImplsPod<[u8; SIZE]>`
///         //
///         // `bound.into_bounds()` extracts the `bounds` field, which is `ImplsPod<T>` here.
///         (infer!(), bound.into_bounds())
///     )
/// }
///
/// const U64: u64 = oned(type_size!(u64));
/// const ONES: [u8; 5] = oned(type_size!([u8; 5]));
///
/// assert_eq!(U64, 0x01_01_01_01_01_01_01_01);
/// assert_eq!(ONES, [1, 1, 1, 1, 1]);
///
/// ```
#[macro_export]
macro_rules! type_size {
    ($ty:ty) => {
        $crate::TypeSize::<_, $ty, { $crate::__::size_of::<$ty>() }>::__NEW__
    };
}

/// Maps the bound field of a [`TypeSize`]
///
/// # Example
///
/// Making a function to repeat a zeroed value, with stronger requirements than it needs.
///
/// ```rust
/// use constmuck::{map_bound, type_size};
/// use constmuck::{ImplsPod, ImplsZeroable, TypeSize, zeroed, zeroed_array};
///
/// use std::num::NonZeroU8;
///
/// pub const fn zeroed_pair<T, const SIZE: usize, const LEN: usize>(
///     bound: TypeSize<ImplsPod<T>, T, SIZE>,
/// ) -> (T, [T; LEN]) {
///     // The type annotation is just for the reader
///     let bound: TypeSize<ImplsZeroable<T>, T, SIZE> =
///         map_bound!(bound, |x| x.impls_zeroable);
///     (zeroed(bound), zeroed_array(bound))
/// }
///
/// const PAIR_U8: (u8, [u8; 4]) = zeroed_pair(type_size!(u8));
///
/// const PAIR_NONE: (Option<NonZeroU8>, [Option<NonZeroU8>; 2]) =
///     zeroed_pair(type_size!(Option<NonZeroU8>));
///
/// assert_eq!(PAIR_U8, (0, [0, 0, 0, 0]));
///
/// assert_eq!(PAIR_NONE, (None, [None, None]));
///
/// ```
#[macro_export]
macro_rules! map_bound {
    ($this:expr, |$bound:ident| $returned:expr $(,)*) => {{
        let ($bound, this) = $crate::TypeSize::split($this);
        this.with_bound($returned)
    }};
    ($this:expr, | $($anything:tt)* ) => {
        compile_error!("expected a closure")
    };
    ($this:expr, $function:expr $(,)*) => {{
        let (bound, this) = $crate::TypeSize::split($this);
        this.with_bound($function(bound))
    }};
}

/// For passing a type along with its size, constructible with the [`type_size`] macro.
///
/// The `B` (bounds) type parameter can be any type that implements [`Infer`],
/// and is implicitly constructed by the [`type_size`] macro.
///
/// # Example
///
/// Making a `max_bit_pattern` function
///
/// ```rust
/// use constmuck::{ImplsPod, TypeSize};
/// use constmuck::{infer, type_size};
///
/// pub const fn max_bit_pattern<T, const SIZE: usize>(bound: TypeSize<ImplsPod<T>, T, SIZE>) -> T {
///     constmuck::cast::<[u8; SIZE], T>(
///         [u8::MAX; SIZE],
///         // `infer!()` here constructs an `ImplsPod<[u8; SIZE]>`
///         //
///         // `bound.into_bounds()` here returns an `ImplsPod<T>`.
///         (infer!(), bound.into_bounds())
///     )
/// }
///
/// const U64: u64 = max_bit_pattern(type_size!(u64));
/// const U8S: [u8; 5] = max_bit_pattern(type_size!([u8; 5]));
/// const I8S: [i8; 5] = max_bit_pattern(type_size!([i8; 5]));
///
/// assert_eq!(U64, u64::MAX);
/// assert_eq!(U8S, [u8::MAX; 5]);
/// assert_eq!(I8S, [-1i8; 5]);
///
/// ```
pub struct TypeSize<B, T, const SIZE: usize> {
    bounds: ManuallyDrop<B>,
    _private: PhantomData<T>,
}

impl<B: Copy, T, const SIZE: usize> Copy for TypeSize<B, T, SIZE> {}

impl<B: Copy, T, const SIZE: usize> Clone for TypeSize<B, T, SIZE> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Infer, T, const SIZE: usize> TypeSize<B, T, SIZE> {
    #[doc(hidden)]
    pub const __NEW__: Self = {
        if mem::size_of::<T>() != SIZE {
            [/* WTF */][mem::size_of::<T>()]
        } else {
            Self {
                bounds: ManuallyDrop::new(Infer::INFER),
                _private: PhantomData,
            }
        }
    };
}

const UNIT_MD: ManuallyDrop<()> = ManuallyDrop::new(());

impl<T, const SIZE: usize> TypeSize<(), T, SIZE> {
    /// Constructs a bound-less `TypeSize`.
    ///
    /// # Safety
    ///
    /// You must ensure that `std::mem::size_of::<T>()` equals the `SIZE` const argument.
    pub const unsafe fn new_unchecked() -> Self {
        Self {
            bounds: UNIT_MD,
            _private: PhantomData,
        }
    }

    /// Constructs a bound-less `TypeSize`.
    ///
    /// # Panics
    ///
    /// Panics if `std::mem::size_of::<T>()` does not equal the `SIZE` const argument.
    pub const fn new_panicking() -> Self {
        if mem::size_of::<T>() == SIZE {
            Self {
                bounds: UNIT_MD,
                _private: PhantomData,
            }
        } else {
            #[allow(non_snake_case)]
            let size_of_T = mem::size_of::<T>();
            [/* size_of::<T>() does not equal SIZE */][size_of_T]
        }
    }
}

impl<T, const SIZE: usize> TypeSize<(), T, SIZE> {
    /// Sets the bounds field of a bound-less `TypeSize`.
    ///
    /// # Leaking
    ///
    /// Note that `B` is expected not to own memory,
    /// dropping a `TypeSize<B, _, _>` will leak any resources it owns.
    pub const fn with_bound<B>(self, bounds: B) -> TypeSize<B, T, SIZE> {
        TypeSize {
            bounds: ManuallyDrop::new(bounds),
            _private: PhantomData,
        }
    }
}

impl<B, T, const SIZE: usize> TypeSize<B, T, SIZE> {
    /// Replaces the bounds field with `bounds`.
    ///
    /// # Leaking
    ///
    /// Note that `V` is expected not to own memory,
    /// dropping a `TypeSize<V, _, _>` will leak any resources it owns.
    pub const fn set_bound<V>(self, bounds: V) -> TypeSize<V, T, SIZE> {
        TypeSize {
            bounds: ManuallyDrop::new(bounds),
            _private: PhantomData,
        }
    }

    /// Turns this TypeSize into its bounds field.
    pub const fn into_bounds(self) -> B {
        ManuallyDrop::into_inner(self.bounds)
    }

    /// Splits this `TypeSize` into its bounds field, and a bound-less `TypeSize`.
    pub const fn split(self) -> (B, TypeSize<(), T, SIZE>) {
        let bounds = ManuallyDrop::into_inner(self.bounds);
        (bounds, TypeSize::__NEW__)
    }
}
