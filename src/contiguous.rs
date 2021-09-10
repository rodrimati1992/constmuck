use bytemuck::Contiguous;

use core::marker::PhantomData;

#[doc(no_inline)]
pub use crate::ImplsContiguous;

pub(crate) mod impls_contiguous {
    use super::*;

    /// Encodes a `T:`[`Contiguous`] bound as a value,
    /// avoids requiring (unstable as of 2021) trait bounds in `const fn`s.
    ///
    pub struct ImplsContiguous<T, IntRepr> {
        pub(super) min_value: IntRepr,
        pub(super) max_value: IntRepr,
        _private: PhantomData<fn() -> (T, IntRepr)>,
    }

    impl<T, IntRepr: Copy> Copy for ImplsContiguous<T, IntRepr> {}

    impl<T, IntRepr: Clone> Clone for ImplsContiguous<T, IntRepr> {
        fn clone(&self) -> Self {
            Self {
                min_value: self.min_value.clone(),
                max_value: self.max_value.clone(),
                _private: PhantomData,
            }
        }
    }

    impl<T: Contiguous> ImplsContiguous<T, T::Int> {
        /// Constructs an `ImplsContiguous`
        ///
        /// You can also use the [`infer`] macro to construct `ImplsContiguous` arguments.
        pub const NEW: Self = Self {
            min_value: T::MIN_VALUE,
            max_value: T::MAX_VALUE,
            _private: PhantomData,
        };
    }

    impl<T, IntRepr> ImplsContiguous<T, IntRepr> {
        /// Gets the minimum value of `T`'s integer representation
        #[inline(always)]
        pub const fn min_value(&self) -> &IntRepr {
            &self.min_value
        }

        /// Gets the maximum value of `T`'s integer representation
        #[inline(always)]
        pub const fn max_value(&self) -> &IntRepr {
            &self.max_value
        }
    }
}

impl<T: Contiguous> crate::Infer for ImplsContiguous<T, T::Int> {
    const INFER: Self = Self::NEW;
}

/// Converts `value: T` into `IntRepr` (its integer representation).
///
/// Requires that `T` implements [`Contiguous<Int = IntRepr>`](bytemuck::Contiguous)
///
/// # Example
///
/// ```
/// use constmuck::{Contiguous, contiguous, infer};
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
/// const BTF: i8 = contiguous::into_integer(Order::BackToFront, infer!());
/// assert_eq!(BTF, 11);
///
/// const RTL: i8 = contiguous::into_integer(Order::RightToLeft, infer!());
/// assert_eq!(RTL, 12);
///
/// const LTR: i8 = contiguous::into_integer(Order::LeftToRight, infer!());
/// assert_eq!(LTR, 13);
///
/// ```
///
#[inline(always)]
pub const fn into_integer<T, IntRepr>(value: T, _bounds: ImplsContiguous<T, IntRepr>) -> IntRepr {
    core::mem::forget(_bounds);

    unsafe { __priv_transmute_unchecked!(T, IntRepr, value) }
}

/// Converts `integer: u8` to `T` if it's between the minimum and maximum values for `T`,
/// otherwise returns `None`.
///
/// Requires that `T` implements [`Contiguous<Int = u8>`](bytemuck::Contiguous)
///
/// # Example
///
/// ### `NonZeroU8`
///
/// ```rust
/// use constmuck::{contiguous, infer};
///
/// use std::num::NonZeroU8;
///
/// const ZERO: Option<NonZeroU8> = contiguous::from_u8_integer(0, infer!());
/// assert_eq!(ZERO, NonZeroU8::new(0));
///
/// const ONE: Option<NonZeroU8> = contiguous::from_u8_integer(1, infer!());
/// assert_eq!(ONE, NonZeroU8::new(1));
///
/// const HUNDRED: Option<NonZeroU8> = contiguous::from_u8_integer(100, infer!());
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
/// const NONE0: Option<Direction> = contiguous::from_u8_integer(0, infer!());
/// assert_eq!(NONE0, None);
///
/// const NONE9: Option<Direction> = contiguous::from_u8_integer(9, infer!());
/// assert_eq!(NONE9, None);
///
/// const UP: Option<Direction> = contiguous::from_u8_integer(10, infer!());
/// assert_eq!(UP, Some(Direction::Up));
///
/// const DOWN: Option<Direction> = contiguous::from_u8_integer(11, infer!());
/// assert_eq!(DOWN, Some(Direction::Down));
///
/// const LEFT: Option<Direction> = contiguous::from_u8_integer(12, infer!());
/// assert_eq!(LEFT, Some(Direction::Left));
///
/// const RIGHT: Option<Direction> = contiguous::from_u8_integer(13, infer!());
/// assert_eq!(RIGHT, Some(Direction::Right));
///
/// const NONE14: Option<Direction> = contiguous::from_u8_integer(14, infer!());
/// assert_eq!(NONE14, None);
///
/// const NONE15: Option<Direction> = contiguous::from_u8_integer(15, infer!());
/// assert_eq!(NONE15, None);
///
/// ```
pub const fn from_u8_integer<T>(integer: u8, bounds: ImplsContiguous<T, u8>) -> Option<T> {
    if bounds.min_value <= integer && integer <= bounds.max_value {
        unsafe { Some(__priv_transmute_from_copy_unchecked!(u8, T, integer)) }
    } else {
        None
    }
}
