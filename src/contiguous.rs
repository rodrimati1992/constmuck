//! Functions for converting types that implement [`Contiguous`]
//! into and from their integer representation.
//!
//! # Example
//!
//! Converting an enum both from and into an integer.
//!
//! ```rust
//! use constmuck::{Contiguous, contiguous};
//!
//! #[repr(u32)]
//! # #[derive(Debug, PartialEq, Copy, Clone)]
//! # /*
//! #[derive(Debug, PartialEq, Contiguous, Copy, Clone)]
//! # */
//! enum Side {
//!     Front = 0,
//!     Back = 1,
//!     Sides = 2,
//! }
//! # unsafe impl Contiguous for Side {
//! #    type Int = u32;
//! #
//! #    const MIN_VALUE: u32 = 0;
//! #    const MAX_VALUE: u32 = 2;
//! # }
//!
//! const SIDE_INTS: [u32; 3] = [
//!     contiguous::into_integer(Side::Front),
//!     contiguous::into_integer(Side::Back),
//!     contiguous::into_integer(Side::Sides),
//! ];
//! assert_eq!(SIDE_INTS, [0, 1, 2]);
//!
//! const SIDE_OPTS: [Option<Side>; 4] = [
//!     contiguous::from_integer(0),
//!     contiguous::from_integer(1),
//!     contiguous::from_integer::<Side>(2),
//!     contiguous::from_integer::<Side>(3),
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

use bytemuck::Contiguous;

use typewit::{HasTypeWitness, MakeTypeWitness, TypeEq, TypeWitnessTypeArg};

use crate::const_panic::PanicVal;

/// Converts `value: T` into `<T as Contiguous>::Int` (its integer representation).
///
/// # Example
///
/// ```
/// use constmuck::{Contiguous, contiguous};
///
/// #[repr(i8)]
/// # #[derive(Debug, PartialEq, Copy, Clone)]
/// # /*
/// #[derive(Debug, PartialEq, Contiguous, Copy, Clone)]
/// # */
/// enum Order {
///     FrontToBack = 10,
///     BackToFront = 11,
///     RightToLeft = 12,
///     LeftToRight = 13,
/// }
/// # unsafe impl Contiguous for Order {
/// #    type Int = i8;
/// #
/// #    const MIN_VALUE: i8 = 10;
/// #    const MAX_VALUE: i8 = 13;
/// # }
///
/// const _ASSERTS: () = {
///     assert!(contiguous::into_integer(Order::FrontToBack) == 10_i8);
///     assert!(contiguous::into_integer(Order::BackToFront) == 11_i8);
///     assert!(contiguous::into_integer(Order::RightToLeft) == 12_i8);
///     assert!(contiguous::into_integer(Order::LeftToRight) == 13_i8);
/// };
/// ```
///
#[inline(always)]
pub const fn into_integer<T: Contiguous>(value: T) -> T::Int {
    // safety:
    // `T: Contiguous` guarantees that `T` is represented as a `T::Int`,
    unsafe { __priv_transmute!(T, T::Int, value) }
}

macro_rules! declare_from_int_fns {
    ($(($fn_name:ident, $variant:ident, $Int:ident))*) => (
        /// Marker trait for the integer types that the
        /// [`from_integer`] function supports.
        ///
        /// This trait can only be implemented in `constmuck`.
        pub trait Integer: Copy + HasTypeWitness<IntegerWit<Self>> {}

        #[allow(missing_debug_implementations)]
        #[doc(hidden)]
        #[non_exhaustive]
        pub enum IntegerWit<W> {
            $(
                $variant(TypeEq<W, $Int>),
            )*
        }

        impl<W> TypeWitnessTypeArg for IntegerWit<W> {
            type Arg = W;
        }

        $(
            impl MakeTypeWitness for IntegerWit<$Int> {
                #[doc(hidden)]
                const MAKE: Self = IntegerWit::$variant(TypeEq::NEW);
            }

            impl Integer for $Int {}
        )*

        #[cold]
        #[inline(never)]
        #[track_caller]
        const fn panic_impossible_bounds(min_value: PanicVal, max_value: PanicVal) -> ! {
            crate::const_panic::concat_panic(&[&[
                PanicVal::write_str("\n\
                    `T` implements `Contiguous` where \
                    `T::MIN_VALUE` is larger than `T::MAX_VALUE`\
                "),
                PanicVal::write_str("\nT::MIN_VALUE: "),
                min_value,
                PanicVal::write_str("\nT::MAX_VALUE: "),
                max_value,
            ]])
        }

        /// Converts an integer into `T` if it's between the minimum and maximum values for `T`,
        /// otherwise returns `None`.
        ///
        /// # Examples
        ///
        /// ### `NonZeroU8`
        ///
        /// ```rust
        /// use constmuck::contiguous;
        ///
        /// use std::num::NonZeroU8;
        ///
        /// const VALUES: [Option<NonZeroU8>; 3] = [
        ///     contiguous::from_integer(0),
        ///     contiguous::from_integer(1),
        ///     contiguous::from_integer::<NonZeroU8>(100),
        /// ];
        ///
        /// assert_eq!(VALUES, [
        ///     None,
        ///     NonZeroU8::new(1),
        ///     NonZeroU8::new(100),
        /// ]);
        ///
        /// ```
        ///
        /// ### Custom type
        ///
        /// ```rust
        /// use constmuck::{Contiguous, contiguous};
        ///
        /// #[repr(u8)]
        /// # #[derive(Debug, PartialEq, Copy, Clone)]
        /// # /*
        /// #[derive(Debug, PartialEq, Contiguous, Copy, Clone)]
        /// # */
        /// enum Direction {
        ///     Up = 10,
        ///     Down = 11,
        ///     Left = 12,
        ///     Right = 13,
        /// }
        /// # unsafe impl Contiguous for Direction {
        /// #    type Int = u8;
        /// #
        /// #    const MIN_VALUE: u8 = 10;
        /// #    const MAX_VALUE: u8 = 13;
        /// # }
        ///
        /// const VALUES: [Option<Direction>; 7] = [
        ///     contiguous::from_integer(0),
        ///     contiguous::from_integer(9),
        ///     contiguous::from_integer(10),
        ///     contiguous::from_integer(11),
        ///     contiguous::from_integer(12),
        ///     contiguous::from_integer(13),
        ///     contiguous::from_integer::<Direction>(14),
        /// ];
        ///
        /// assert_eq!(VALUES, [
        ///     None,
        ///     None,
        ///     Some(Direction::Up),
        ///     Some(Direction::Down),
        ///     Some(Direction::Left),
        ///     Some(Direction::Right),
        ///     None,
        /// ]);
        ///
        /// ```
        #[track_caller]
        pub const fn from_integer<T: Contiguous>(integer: T::Int) -> Option<T>
        where
            T::Int: Integer
        {
            match <T::Int>::WITNESS {
                $(
                    IntegerWit::$variant(te) => {
                        let integer = te.to_right(integer);
                        let min_value = te.to_right(T::MIN_VALUE);
                        let max_value = te.to_right(T::MAX_VALUE);

                        #[cfg(debug_assertions)]
                        if min_value > max_value {
                            use crate::const_panic::{FmtArg as FA};

                            panic_impossible_bounds(
                                PanicVal::$fn_name(min_value, FA::DEBUG),
                                PanicVal::$fn_name(max_value, FA::DEBUG),
                            );
                        }

                        if integer < min_value || max_value < integer {
                            return None;
                        }
                    }
                )*
            }

            // safety:
            // integer is between `T::MIN_VALUE` and `T::MAX_VALUE`(inclusive).
            //
            // `T: Contiguous` guarantees that `T` is represented as a `T::Int`,
            // and is valid for all values between `T::MIN_VALUE` and
            // `T::MAX_VALUE` inclusive.
            unsafe { Some(__priv_transmute_from_copy!(T::Int, T, integer)) }
        }
    );
}

declare_from_int_fns! {
    (from_i8,    I8,    i8)
    (from_i16,   I16,   i16)
    (from_i32,   I32,   i32)
    (from_i64,   I64,   i64)
    (from_i128,  I128,  i128)
    (from_isize, Isize, isize)
    (from_u8,    U8,    u8)
    (from_u16,   U16,   u16)
    (from_u32,   U32,   u32)
    (from_u64,   U64,   u64)
    (from_u128,  U128,  u128)
    (from_usize, Usize, usize)
}
