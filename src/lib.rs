//! Const equivalents of many [`bytemuck`] functions.
//!
//! `constmuck` uses [`bytemuck`'s traits],
//! any type that implements those traits can be used with the
//! relevant functions from this crate.
//!
//! Because the `*_alt` functions are `const fn`s,
//! they can't inspect the address of the reference parameter.
//! This differs from their [`bytemuck`] equivalents,
//! which use the address to determine alignment.
//!
//! # Examples
//!
//! These examples use bytemuck's derives to show how users don't need to
//! write `unsafe` to use this crate,
//! and use the [`konst`] crate to make writing the const functions easier.
//!
//! ### Contiguous
//!
//! This example demonstrates constructing an enum from its representation.
//!
//! ```rust
//!
//! use constmuck::Contiguous;
//!
//! use konst::{array, try_opt};
//!
//! fn main() {
//!     const COLORS: Option<[Color; 5]> = Color::from_array([3, 4, 1, 0, 2]);
//!     assert_eq!(
//!         COLORS,
//!         Some([Color::White, Color::Black, Color::Blue, Color::Red, Color::Green]),
//!     );
//!
//!     const NONE_COLORS: Option<[Color; 4]> = Color::from_array([1, 2, 3, 5]);
//!     assert_eq!(NONE_COLORS, None);
//! }
//!
//! #[repr(u8)]
//! #[derive(Debug, PartialEq, Eq, Contiguous, Copy, Clone)]
//! pub enum Color {
//!     Red = 0,
//!     Blue,
//!     Green,
//!     White,
//!     Black,
//! }
//!
//! impl Color {
//!     pub const fn from_int(n: u8) -> Option<Self> {
//!         constmuck::contiguous::from_integer(n)
//!     }
//!     pub const fn from_array<const N: usize>(input: [u8; N]) -> Option<[Self; N]> {
//!         // `try_opt` returns from `from_array` on `None`,
//!         // because `konst::array::map` allows the passed-in expression
//!         // to return from the surrounding named function.
//!         Some(array::map!(input, |n| try_opt!(Self::from_int(n))))
//!     }
//! }
//!
//!
//! ```
//!
//! ### Wrapper
//!
//! This example demonstrates a type that wraps a `[T]`, constructed by reference.
//!
//! ```rust
//!
//! use constmuck::TransparentWrapper;
//!
//! fn main() {
//!     const SLICE: &[u32] = &[3, 5, 8, 13, 21];
//!     const WRAPPER: &SliceWrapper<u32> = SliceWrapper::new(SLICE);
//!
//!     const SUM: u64 = WRAPPER.sum();
//!     assert_eq!(SUM, 50);
//!
//!     const FIRST_EVEN: Option<(usize, u32)> = WRAPPER.find_first_even();
//!     assert_eq!(FIRST_EVEN, Some((2, 8)));
//! }
//!
//! #[repr(transparent)]
//! #[derive(Debug, PartialEq, Eq, TransparentWrapper)]
//! pub struct SliceWrapper<T>(pub [T]);
//!
//! impl<T> SliceWrapper<T> {
//!     // Using `constmuck` allows safely defining this function as a `const fn`
//!     pub const fn new(reff: &[T]) -> &Self {
//!         constmuck::wrapper::wrap_ref!(reff)
//!     }
//! }
//!
//! impl SliceWrapper<u32> {
//!     pub const fn sum(&self) -> u64 {
//!         konst::iter::eval!(&self.0,copied(),fold(0, |l, r| l + r as u64))
//!     }
//!     pub const fn find_first_even(&self) -> Option<(usize, u32)> {
//!         konst::iter::eval!(&self.0,copied(),enumerate(),find(|(i, n)| *n % 2 == 0))
//!     }
//! }
//!
//!
//! ```
//!
//! # Additional checks
//!
//! The `"debug_checks"` crate feature (which is disabled by default)
//! enables additional assertions in `constmuck` functions,
//! these assertions panic in some cases where unsound impls of [`bytemuck`] traits
//! would have caused Undefined Behavior.
//!
//! # Features
//!
//! These are the features of this crate:
//!
//! - `"derive"`(disabled by default):
//! Enables `bytemuck`'s `"derive"` feature and reexports its derives.
//!
//! - `"debug_checks"`(disabled by default):
//! Enables [additional safety checks](#additional-checks) for detecting some
//! Undefined Behavior.
//!
//! # No-std support
//!
//! `constmuck` is `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `constmuck` requires Rust 1.65.0.
//!
//! You can use the `"rust_latest_stable"` crate feature to get
//! all items and functionality that requires stable Rust versions after 1.65.0.
//!
//! [`bytemuck`]: bytemuck
//! [`bytemuck`'s traits]: bytemuck#traits
//! [`konst`]: https://docs.rs/konst/0.3/konst/index.html
//! [`contiguous`]: ./contiguous/index.html
//! [`wrapper`]: ./wrapper/index.html

#![no_std]
#![deny(unused_results)]
#![deny(clippy::missing_safety_doc)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeTest;

#[macro_use]
mod macros;

pub mod contiguous;

mod pod;

mod slice_fns;

pub mod wrapper;

mod zeroable;

#[doc(hidden)]
pub mod __priv_utils;

#[doc(no_inline)]
pub use bytemuck::{
    self, AnyBitPattern, Contiguous, NoUninit, Pod, PodCastError, TransparentWrapper, Zeroable,
};

pub use crate::{
    pod::{
        cast, cast_ref_alt, pod_read_unaligned, try_cast, try_cast_ref_alt, try_pod_read_unaligned,
    },
    slice_fns::{bytes_of, cast_slice_alt, try_cast_slice_alt},
    zeroable::zeroed,
};

#[doc(hidden)]
pub mod __ {
    pub use core::mem::size_of;
    pub use core::ops::Range;
}

use constmuck_internal::const_panic;
