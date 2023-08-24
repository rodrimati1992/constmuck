use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::{self, ManuallyDrop},
};

use crate::{Infer, IsCopy};

/// Constructs a [`TypeSize<$ty, $bounds, _>`](struct@crate::TypeSize),
#[macro_export]
macro_rules! TypeSize {
    ($ty:ty $(,)*) => {
        $crate::TypeSize::<$ty, _, { $crate::__::size_of::<$ty>() }>::__13878307735224946849NEW__
    };
    ($ty:ty, $bounds:ty $(,)*) => {
        $crate::TypeSize::<$ty, $bounds, {$crate::__::size_of::<$ty>()}>::__13878307735224946849NEW__
    };
}

/// Maps the bound field of a [`TypeSize`](struct@crate::TypeSize).
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

/// For passing a type along with its size and additional bounds.
pub struct TypeSize<T, B, const SIZE: usize> {
    bounds: ManuallyDrop<B>,
    // The lifetime of `T` is invariant,
    // just in case that it's unsound for lifetimes to be co/contravariant.
    _private: PhantomData<fn(T) -> T>,
}

impl<T, B: Debug, const SIZE: usize> Debug for TypeSize<T, B, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypeSize")
            .field("bounds", &self.bounds)
            .finish()
    }
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

impl<T, B, const SIZE: usize> TypeSize<T, B, SIZE> {
    const __GHOST: PhantomData<fn(T) -> T> = PhantomData;
}

impl<T, const SIZE: usize> TypeSize<T, (), SIZE> {
    const __UNCHECKED_UNIT: Self = Self {
        bounds: ManuallyDrop::new(()),
        _private: PhantomData,
    };

    /// Constructs a bound-less `TypeSize`.
    ///
    /// # Safety
    ///
    /// You must ensure that `std::mem::size_of::<T>()` equals the `SIZE` const argument.
    #[inline(always)]
    pub const unsafe fn new_unchecked() -> Self {
        Self::__UNCHECKED_UNIT
    }

    /// Constructs a bound-less `TypeSize`.
    ///
    /// # Panics
    ///
    /// Panics if `std::mem::size_of::<T>()` does not equal the `SIZE` const argument.
    pub const fn new_panicking() -> Self {
        if mem::size_of::<T>() == SIZE {
            Self::__UNCHECKED_UNIT
        } else {
            panic!()
        }
    }
}

impl<T, const SIZE: usize> TypeSize<T, (), SIZE> {
    /// Sets the bounds field of a bound-less `TypeSize`.
    pub const fn with_bounds<B>(self, bounds: B) -> TypeSize<T, B, SIZE> {
        TypeSize {
            bounds: ManuallyDrop::new(bounds),
            _private: Self::__GHOST,
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
            _private: Self::__GHOST,
        }
    }

    /// Accessor for the bounds field.
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
    #[inline(always)]
    pub const fn repeat<const LEN: usize>(self, reff: &T) -> [T; LEN] {
        crate::copying::repeat(reff, self)
    }
}
