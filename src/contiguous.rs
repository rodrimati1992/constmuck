//! Functions for converting types that implement [`Contiguous`]
//! into and from their integer representation.
//!
//! Related: the [`IsContiguous`](struct@crate::IsContiguous) type.
//!
//! # Example
//!
//! Converting an enum both from and into an integer.
//!
//! ```rust
//! use constmuck::{Contiguous, IsContiguous, contiguous, infer};
//!
//! #[repr(u32)]
//! #[derive(Debug, PartialEq, Copy, Clone)]
//! enum Side {
//!     Front = 0,
//!     Back = 1,
//!     Sides = 2,
//! }
//!
//! unsafe impl Contiguous for Side {
//!    type Int = u32;
//!
//!    const MIN_VALUE: u32 = 0;
//!    const MAX_VALUE: u32 = 2;
//! }
//!
//! const SIDE_INTS: [u32; 3] = [
//!     contiguous::into_integer(Side::Front, infer!()),
//!     contiguous::into_integer(Side::Back, infer!()),
//!     contiguous::into_integer(Side::Sides, infer!()),
//! ];
//! assert_eq!(SIDE_INTS, [0, 1, 2]);
//!
//! const SIDE_OPTS: [Option<Side>; 4] = [
//!     contiguous::from_u32(0, infer!()),
//!     contiguous::from_u32(1, IsContiguous!()),
//!     contiguous::from_u32(2, IsContiguous!(Side)),
//!     contiguous::from_u32(3, IsContiguous!(Side, u32)),
//! ];
//!
//! assert_eq!(
//!     SIDE_OPTS,
//!     [Some(Side::Front), Some(Side::Back), Some(Side::Sides), None],
//! );
//!
//!
//! ```
//!

/// Constructs an [`IsContiguous<$T, $IntRepr>`](struct@crate::IsContiguous),
/// requiring that `$T` implements [`Contiguous`](trait@bytemuck::Contiguous).
///
/// This has two optional type arguments (`$T` and `$IntRepr`) that default to
/// infering the type if not passed.
///
/// # Example
///
/// ```rust
/// use constmuck::{IsContiguous, contiguous};
///
/// use std::num::NonZeroU8;
///
/// // The three lines below are equivalent.
/// const FOO: IsContiguous<NonZeroU8, u8> = IsContiguous!();
/// const BAR: IsContiguous<NonZeroU8, u8> = IsContiguous!(NonZeroU8);
/// const BAZ: IsContiguous<NonZeroU8, u8> = IsContiguous!(NonZeroU8, u8);
///
/// assert_eq!(contiguous::from_u8(0, FOO), None);
/// assert_eq!(contiguous::from_u8(0, BAR), None);
/// assert_eq!(contiguous::from_u8(0, BAZ), None);
/// assert_eq!(contiguous::from_u8(1, BAZ), Some(NonZeroU8::new(1).unwrap()));
///
/// assert_eq!(contiguous::into_integer(NonZeroU8::new(1).unwrap(), FOO), 1u8);
///
///
/// ```
#[macro_export]
macro_rules! IsContiguous {
    () => {
        <$crate::IsContiguous<_, _> as $crate::Infer>::INFER
    };
    ($T:ty $(,)*) => {
        <$crate::IsContiguous<$T, _> as $crate::Infer>::INFER
    };
    ($T:ty, $IntRepr:ty $(,)*) => {
        <$crate::IsContiguous<$T, $IntRepr> as $crate::Infer>::INFER
    };
}

use bytemuck::Contiguous;

use core::marker::PhantomData;

#[doc(no_inline)]
pub use crate::IsContiguous;

pub(crate) mod is_contiguous {
    use super::*;

    /// Encodes a `T:`[`Contiguous`] bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    /// This also stores the [minimum](Self::min_value) and [maximum](Self::max_value)
    /// values of the integer represetantion.
    ///
    /// Related: the [`contiguous`](crate::contiguous) module.
    pub struct IsContiguous<T, IntRepr> {
        pub(super) min_value: IntRepr,
        pub(super) max_value: IntRepr,
        _private: PhantomData<fn() -> (T, IntRepr)>,
    }

    impl<T, IntRepr: Copy> Copy for IsContiguous<T, IntRepr> {}

    impl<T, IntRepr: Clone> Clone for IsContiguous<T, IntRepr> {
        fn clone(&self) -> Self {
            Self {
                min_value: self.min_value.clone(),
                max_value: self.max_value.clone(),
                _private: PhantomData,
            }
        }
    }

    impl<T: Contiguous> IsContiguous<T, T::Int> {
        /// Constructs an `IsContiguous`
        ///
        /// You can also use the [`infer`] macro to construct `IsContiguous` arguments.
        pub const NEW: Self = Self {
            min_value: T::MIN_VALUE,
            max_value: T::MAX_VALUE,
            _private: PhantomData,
        };
    }

    impl<T, IntRepr> IsContiguous<T, IntRepr> {
        const __PHANTOM__: PhantomData<fn() -> (T, IntRepr)> = PhantomData;

        /// Constructs an `IsContiguous` without checking that `T` implements
        /// [`Contiguous<Int = IntRepr>`](bytemuck::Contiguous)
        ///
        /// # Safety
        ///
        /// You must ensure that `T` follows the
        /// [safety requirements of `Contiguous`](bytemuck::Contiguous#safety)
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::{IsContiguous, contiguous};
        ///
        /// use std::num::Wrapping;
        ///
        /// let ic = unsafe { IsContiguous::<Wrapping<u8>, u8>::new_unchecked(10, 20) };
        ///
        /// assert_eq!(contiguous::from_u8(9, ic), None);
        /// assert_eq!(contiguous::from_u8(10, ic), Some(Wrapping(10)));
        /// assert_eq!(contiguous::from_u8(20, ic), Some(Wrapping(20)));
        /// assert_eq!(contiguous::from_u8(21, ic), None);
        ///
        /// assert_eq!(contiguous::into_integer(Wrapping(11), ic), 11);
        /// assert_eq!(contiguous::into_integer(Wrapping(15), ic), 15);
        ///
        ///
        /// ```
        pub const unsafe fn new_unchecked(min_value: IntRepr, max_value: IntRepr) -> Self {
            Self {
                min_value,
                max_value,
                _private: Self::__PHANTOM__,
            }
        }
    }
    impl<T, IntRepr> IsContiguous<T, IntRepr> {
        /// Gets the minimum value of `T`'s integer representation
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::IsContiguous;
        ///
        /// use std::num::NonZeroU8;
        ///
        /// {
        ///     let ic = IsContiguous!(NonZeroU8);
        ///     assert_eq!(ic.min_value(), &1);
        /// }
        /// {
        ///     let ic = IsContiguous!(u16);
        ///     assert_eq!(ic.min_value(), &0);
        /// }
        /// ```
        #[inline(always)]
        pub const fn min_value(&self) -> &IntRepr {
            &self.min_value
        }

        /// Gets the maximum value of `T`'s integer representation
        ///
        /// # Example
        ///
        /// ```rust
        /// use constmuck::IsContiguous;
        ///
        /// use std::num::NonZeroU16;
        ///
        /// {
        ///     let ic = IsContiguous!(NonZeroU16);
        ///     assert_eq!(ic.max_value(), &u16::MAX);
        /// }
        /// {
        ///     let ic = IsContiguous!(u8);
        ///     assert_eq!(ic.max_value(), &u8::MAX);
        /// }
        /// ```
        #[inline(always)]
        pub const fn max_value(&self) -> &IntRepr {
            &self.max_value
        }
    }
}

impl<T: Contiguous> crate::Infer for IsContiguous<T, T::Int> {
    const INFER: Self = Self::NEW;
}

/// Converts `value: T` into `IntRepr` (its integer representation).
///
/// Requires that `T` implements [`Contiguous<Int = IntRepr>`](bytemuck::Contiguous)
///
/// # Example
///
/// ```
/// use constmuck::{Contiguous, IsContiguous, contiguous, infer};
///
/// #[repr(i8)]
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Order {
///     FrontToBack = 10,
///     BackToFront = 11,
///     RightToLeft = 12,
///     LeftToRight = 13,
/// }
///
/// unsafe impl Contiguous for Order {
///    type Int = i8;
///
///    const MIN_VALUE: i8 = 10;
///    const MAX_VALUE: i8 = 13;
/// }
///
///
/// const FTB: i8 = contiguous::into_integer(Order::FrontToBack, infer!());
/// assert_eq!(FTB, 10);
///
/// const BTF: i8 = contiguous::into_integer(Order::BackToFront, IsContiguous!());
/// assert_eq!(BTF, 11);
///
/// const RTL: i8 = contiguous::into_integer(Order::RightToLeft, IsContiguous!(Order));
/// assert_eq!(RTL, 12);
///
/// const LTR: i8 = contiguous::into_integer(Order::LeftToRight, IsContiguous!(Order, i8));
/// assert_eq!(LTR, 13);
///
/// ```
///
#[inline(always)]
pub const fn into_integer<T, IntRepr>(value: T, _bounds: IsContiguous<T, IntRepr>) -> IntRepr {
    core::mem::forget(_bounds);

    unsafe { __priv_transmute!(T, IntRepr, value) }
}

/// Converts `integer: u8` to `T` if it's between the minimum and maximum values for `T`,
/// otherwise returns `None`.
///
/// Requires that `T` implements [`Contiguous<Int = u8>`](bytemuck::Contiguous)
///
/// # Examples
///
/// ### `NonZeroU8`
///
/// ```rust
/// use constmuck::{IsContiguous, contiguous, infer};
///
/// use std::num::NonZeroU8;
///
/// const ZERO: Option<NonZeroU8> = contiguous::from_u8(0, infer!());
/// assert_eq!(ZERO, None);
///
/// const ONE: Option<NonZeroU8> = contiguous::from_u8(1, IsContiguous!());
/// assert_eq!(ONE, NonZeroU8::new(1));
///
/// const HUNDRED: Option<NonZeroU8> = contiguous::from_u8(100, IsContiguous!(NonZeroU8));
/// assert_eq!(HUNDRED, NonZeroU8::new(100));
///
/// ```
///
/// ### Custom type
///
/// ```rust
/// use constmuck::{Contiguous, contiguous, infer};
///
/// #[repr(u8)]
/// #[derive(Debug, PartialEq, Copy, Clone)]
/// enum Direction {
///     Up = 10,
///     Down = 11,
///     Left = 12,
///     Right = 13,
/// }
///
/// unsafe impl Contiguous for Direction {
///    type Int = u8;
///
///    const MIN_VALUE: u8 = 10;
///    const MAX_VALUE: u8 = 13;
/// }
///
///
/// const NONE0: Option<Direction> = contiguous::from_u8(0, infer!());
/// assert_eq!(NONE0, None);
///
/// const NONE9: Option<Direction> = contiguous::from_u8(9, infer!());
/// assert_eq!(NONE9, None);
///
/// const UP: Option<Direction> = contiguous::from_u8(10, infer!());
/// assert_eq!(UP, Some(Direction::Up));
///
/// const DOWN: Option<Direction> = contiguous::from_u8(11, infer!());
/// assert_eq!(DOWN, Some(Direction::Down));
///
/// const LEFT: Option<Direction> = contiguous::from_u8(12, infer!());
/// assert_eq!(LEFT, Some(Direction::Left));
///
/// const RIGHT: Option<Direction> = contiguous::from_u8(13, infer!());
/// assert_eq!(RIGHT, Some(Direction::Right));
///
/// const NONE14: Option<Direction> = contiguous::from_u8(14, infer!());
/// assert_eq!(NONE14, None);
///
/// ```
pub const fn from_u8<T>(integer: u8, bounds: IsContiguous<T, u8>) -> Option<T> {
    #[cfg(feature = "debug_checks")]
    #[allow(unconditional_panic)]
    if bounds.min_value > bounds.max_value {
        let x = 0;
        let _: () = [/* bounds.min_value is larger than bounds.max_value */][x];
    }

    if bounds.min_value <= integer && integer <= bounds.max_value {
        unsafe { Some(__priv_transmute_from_copy!(u8, T, integer)) }
    } else {
        None
    }
}

macro_rules! declare_from_integer_fns {
    ($(($fn_name:ident, $Int:ident))*) => (
        declare_from_integer_fns!{
            @inner
            $((
                $fn_name,
                $Int,
                concat!(
                    "Converts `ìnteger: ", stringify!($Int), "` to `T` if it's between ",
                    "the minimum and maximum values for `T`, otherwise returns `None`.\n\n",
                    "Requires that `T` implements [`Contiguous<Int = ", stringify!($Int),
                    ">`](bytemuck::Contiguous)"
                )
            ))*
        }
    );
    (@inner $(($fn_name:ident, $Int:ident, $shared_doc:expr))*)=>{
        $(
            impl<T> FromInteger<T, $Int> {
                #[doc = $shared_doc]
                #[inline(always)]
                pub const fn call(self) -> Option<T> {
                    $fn_name(self.0, self.1)
                }
            }
        )*

        $(
            declare_from_integer_fns!{@free_fn $fn_name, $Int, $shared_doc}
        )*
    };
    (@free_fn from_u8, $Int:ident, $shared_doc:expr)=>{};
    (@free_fn $fn_name:ident, $Int:ident, $shared_doc:expr)=>{
        #[doc = $shared_doc]
        /// # Examples
        ///
        /// For examples, you can look
        /// [at the ones for `from_u8`](self::from_u8#examples).
        ///
        pub const fn $fn_name<T>(integer: $Int, bounds: IsContiguous<T, $Int>) -> Option<T> {
            #[cfg(feature = "debug_checks")]
            #[allow(unconditional_panic)]
            if bounds.min_value > bounds.max_value {
                let x = 0;
                let _: () = [/* bounds.min_value is larger than bounds.max_value */][x];
            }

            if bounds.min_value <= integer && integer <= bounds.max_value {
                unsafe { Some(__priv_transmute_from_copy!($Int, T, integer)) }
            } else {
                None
            }
        }
    };
}

declare_from_integer_fns! {
    (from_i8, i8)
    (from_i16, i16)
    (from_i32, i32)
    (from_i64, i64)
    (from_i128, i128)
    (from_isize, isize)
    (from_u8, u8)
    (from_u16, u16)
    (from_u32, u32)
    (from_u64, u64)
    (from_u128, u128)
    (from_usize, usize)
}

/// Converts `IntRepr` to `T` if it's between the minimum and maximum values for `T`,
/// otherwise returns `None`.
///
/// This is only useful over the functions in the [`contiguous`](crate::contiguous)
/// module when one needs to select the method based on the type of the integer.
///
/// # Limitation
///
/// The concrete type of the integer must be known for the `call` method to be callable,
/// it can't be inferred from the type that it's converted into.
///
/// # Example
///
/// ```rust
/// use constmuck::contiguous::FromInteger;
/// use constmuck::{IsContiguous, infer};
///
/// use std::num::{NonZeroU32, NonZeroUsize};
///
/// const ZERO_USIZE: Option<NonZeroUsize> =
///     FromInteger(0usize, infer!()).call();
/// assert_eq!(ZERO_USIZE, None);
///
/// const TWO_USIZE: Option<NonZeroUsize> =
///     FromInteger(2usize, IsContiguous!()).call();
/// assert_eq!(TWO_USIZE, NonZeroUsize::new(2));
///
///
/// const ZERO_U64: Option<NonZeroU32> =
///     FromInteger(0u32, IsContiguous!(NonZeroU32)).call();
/// assert_eq!(ZERO_U64, None);
///
/// const ONE_U64: Option<NonZeroU32> =
///     FromInteger(1u32, IsContiguous!(NonZeroU32, u32)).call();
/// assert_eq!(ONE_U64, NonZeroU32::new(1));
///
///
/// ```
///
pub struct FromInteger<T, IntRepr>(pub IntRepr, pub IsContiguous<T, IntRepr>);
