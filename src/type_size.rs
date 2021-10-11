use core::{
    marker::PhantomData,
    mem::{self, ManuallyDrop},
};

use crate::{Infer, IsCopy, IsZeroable};

/// Constructs a [`TypeSize<$ty, $bounds, _>`](struct@crate::TypeSize),
///
/// Uses the [`Infer`] trait to construct a `$bounds`.
///
/// The `$bounds` type argument is optional, defaulting to being inferred.
///
/// # Example
///
/// Making a `oned` function
///
/// ```rust
/// use constmuck::{IsPod, TypeSize};
///
/// pub const fn oned<T, const SIZE: usize>(bound: TypeSize<T, IsPod<T>, SIZE>) -> T {
///     constmuck::cast::<[u8; SIZE], T>(
///         [1; SIZE],
///         // `IsPod!()` here constructs an `IsPod<[u8; SIZE]>`
///         //
///         // `bound.into_bounds()` extracts the `bounds` field, which is `IsPod<T>` here.
///         (IsPod!(), bound.into_bounds())
///     )
/// }
///
/// const U64: u64 = oned(TypeSize!(u64));
///
/// // Passing the `$bounds` type argument explicitly
/// const ONES: [u8; 5] = oned(TypeSize!([u8; 5], IsPod<[u8; 5]>));
///
/// assert_eq!(U64, 0x01_01_01_01_01_01_01_01);
/// assert_eq!(ONES, [1, 1, 1, 1, 1]);
///
/// ```
#[macro_export]
macro_rules! TypeSize {
    ($ty:ty $(,)*) => {
        $crate::TypeSize::<$ty, _, { $crate::__::size_of::<$ty>() }>::__13878307735224946849NEW__
    };
    ($ty:ty, $bounds:ty $(,)*) => {
        $crate::TypeSize::<
                    $ty,
                    $bounds,
                    { $crate::__::size_of::<$ty>() }
                >::__13878307735224946849NEW__
    };
}

/// Maps the bound field of a [`TypeSize`](struct@crate::TypeSize).
///
/// # Example
///
/// Making a function to repeat a zeroed value, with stronger requirements than it needs.
///
/// ```rust
/// use constmuck::map_bound;
/// use constmuck::{IsPod, IsZeroable, TypeSize, zeroed, zeroed_array};
///
/// use std::num::NonZeroU8;
///
/// pub const fn zeroed_pair<T, const SIZE: usize, const LEN: usize>(
///     bound: TypeSize<T, IsPod<T>, SIZE>,
/// ) -> (T, [T; LEN]) {
///     // The type annotation is just for the reader
///     let bound: TypeSize<T, IsZeroable<T>, SIZE> =
///         map_bound!(bound, |x| x.is_zeroable);
///     (zeroed(bound), zeroed_array(bound))
/// }
///
/// const PAIR_U8: (u8, [u8; 4]) = zeroed_pair(TypeSize!(u8));
///
/// const PAIR_NONE: (Option<NonZeroU8>, [Option<NonZeroU8>; 2]) =
///     zeroed_pair(TypeSize!(Option<NonZeroU8>));
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
        this.with_bounds($returned)
    }};
    ($this:expr, | $($anything:tt)* ) => {
        compile_error!("expected a closure")
    };
    ($this:expr, $function:expr $(,)*) => {{
        let (bound, this) = $crate::TypeSize::split($this);
        this.with_bounds($function(bound))
    }};
}

/// For passing a type along with its size, constructible with the [`TypeSize`] macro.
///
/// The `B` (bounds) type parameter can be any type that implements [`Infer`],
/// and is implicitly constructed by the [`TypeSize`] macro.
///
/// # Example
///
/// Making a `max_bit_pattern` function
///
/// ```rust
/// use constmuck::{IsPod, TypeSize};
///
/// pub const fn max_bit_pattern<T, const SIZE: usize>(bound: TypeSize<T, IsPod<T>, SIZE>) -> T {
///     constmuck::cast::<[u8; SIZE], T>(
///         [u8::MAX; SIZE],
///         // `IsPod!()` here constructs an `IsPod<[u8; SIZE]>`
///         //
///         // `bound.bounds()` here returns a `&IsPod<T>`.
///         (IsPod!(), *bound.bounds())
///     )
/// }
///
/// const U64: u64 = max_bit_pattern(TypeSize!(u64));
/// const U8S: [u8; 5] = max_bit_pattern(TypeSize!([u8; 5]));
/// const I8S: [i8; 5] = max_bit_pattern(TypeSize!([i8; 5]));
///
/// assert_eq!(U64, u64::MAX);
/// assert_eq!(U8S, [u8::MAX; 5]);
/// assert_eq!(I8S, [-1i8; 5]);
///
/// ```
///
/// [`TypeSize`]: macro@crate::TypeSize
pub struct TypeSize<T, B, const SIZE: usize> {
    bounds: ManuallyDrop<B>,
    _private: PhantomData<T>,
}

impl<T, B: Copy, const SIZE: usize> Copy for TypeSize<T, B, SIZE> {}

impl<T, B: Copy, const SIZE: usize> Clone for TypeSize<T, B, SIZE> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, B: Infer, const SIZE: usize> TypeSize<T, B, SIZE> {
    #[doc(hidden)]
    pub const __13878307735224946849NEW__: Self = {
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

impl<T, const SIZE: usize> TypeSize<T, (), SIZE> {
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

impl<T, const SIZE: usize> TypeSize<T, (), SIZE> {
    /// Sets the bounds field of a bound-less `TypeSize`.
    ///
    /// # Leaking
    ///
    /// Note that `B` is expected not to own memory,
    /// dropping a `TypeSize<_, B, _>` will leak any resources it owns.
    ///
    /// # Example
    ///
    /// This example demonstrates how `with_bounds` can be used to
    /// compose `TypeSize` with `Is*::new_unchecked`.
    ///
    /// ```rust
    /// use constmuck::{IsZeroable, TypeSize, zeroed};
    ///
    /// fn main() {
    ///     const NEW: Foo = Foo::new();
    ///     
    ///     assert_eq!(NEW, Foo(0, 0, 0));
    /// }
    ///
    /// #[derive(Debug, PartialEq)]
    /// pub struct Foo(u8, u16, u32);
    ///
    /// impl Foo {
    ///     pub const fn new() -> Self {
    ///         // safety: this type knows that all its fields are zeroable right now,
    ///         // but it doesn't impl Zeroable to be able to add nonzeroable fields.
    ///         let iz = unsafe{ IsZeroable::<Self>::new_unchecked() };
    ///         zeroed(TypeSize!(Self).with_bounds(iz))
    ///     }
    /// }
    ///
    /// ```
    pub const fn with_bounds<B>(self, bounds: B) -> TypeSize<T, B, SIZE> {
        TypeSize {
            bounds: ManuallyDrop::new(bounds),
            _private: PhantomData,
        }
    }
}

impl<B, T, const SIZE: usize> TypeSize<T, B, SIZE> {
    /// Replaces the bounds field with `bounds`.
    ///
    /// # Leaking
    ///
    /// Note that `V` is expected not to own memory,
    /// dropping a `TypeSize<_, V, _>` will leak any resources it owns.
    pub const fn set_bounds<V>(self, bounds: V) -> TypeSize<T, V, SIZE> {
        TypeSize {
            bounds: ManuallyDrop::new(bounds),
            _private: PhantomData,
        }
    }

    /// Gets a reference to the bounds field.
    pub const fn bounds(&self) -> &B {
        crate::__priv_utils::manuallydrop_as_inner(&self.bounds)
    }

    /// Turns this `TypeSize` into its bounds field.
    pub const fn into_bounds(self) -> B {
        ManuallyDrop::into_inner(self.bounds)
    }

    /// Splits this `TypeSize` into its bounds field, and a bound-less `TypeSize`.
    pub const fn split(self) -> (B, TypeSize<T, (), SIZE>) {
        let bounds = ManuallyDrop::into_inner(self.bounds);
        (bounds, TypeSize::__13878307735224946849NEW__)
    }
}

impl<T, const SIZE: usize> TypeSize<T, IsCopy<T>, SIZE> {
    /// Equivalent to [`copying::repeat`](crate::copying::repeat)
    /// but allows passing the length of the retuned array.
    ///
    /// Creates a `[T; ARR_LEN]` by copying from a `&T`
    ///
    /// Requires that `T` implements `Copy + Pod`
    /// (see [`IsCopy`] docs for why it requires `Pod`)
    ///
    /// # Example
    ///
    /// ```rust
    /// use constmuck::TypeSize;
    ///
    /// const PAIR: &[u128] = &TypeSize!(u128).repeat::<2>(&300);
    ///
    /// assert_eq!(PAIR, &[300, 300]);
    ///
    /// ```
    #[inline(always)]
    pub const fn repeat<const LEN: usize>(self, reff: &T) -> [T; LEN] {
        crate::copying::repeat(reff, self)
    }
}

impl<T, const SIZE: usize> TypeSize<T, IsZeroable<T>, SIZE> {
    /// Equivalent to [`constmuck::zeroed_array`](crate::zeroed_array)
    /// but allows passing the length of the retuned array.
    ///
    /// For safely getting a [`std::mem::zeroed`](core::mem::zeroed) `[T; N]`.
    ///
    /// This function requires that `T` implements [`Zeroable`](bytemuck::Zeroable).
    ///
    /// # Example
    ///
    /// ```rust
    /// use constmuck::{TypeSize, zeroed_array};
    ///
    /// const BYTES: &[u8] = &TypeSize!(u8).zeroed_array::<2>();
    /// const CHARS: &[char] = &TypeSize!(char).zeroed_array::<4>();
    ///
    /// assert_eq!(BYTES, [0, 0]);
    /// assert_eq!(CHARS, ['\0', '\0', '\0', '\0']);
    ///
    ///
    /// ```
    #[inline(always)]
    pub const fn zeroed_array<const LEN: usize>(self) -> [T; LEN] {
        crate::zeroed_array(self)
    }
}
