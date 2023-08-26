//! Const equivalents of many [`bytemuck`] functions,
//! and additional functionality.
//!
//! `constmuck` uses `bytemuck`'s traits,
//! any type that implements those traits can be used with the
//! relevant functions from this crate.
//!
//! The `*_alt` functions aren't exactly equivalent to the `bytemuck` ones,
//! each one describes how it's different.
//!
//! This crate avoids requiring (unstable as of 2021) trait bounds in `const fn`s
//! by using marker types to require that a trait is implemented.
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
//! # #[derive(Debug, PartialEq, Eq, Copy, Clone)]
//! # /*
//! #[derive(Debug, PartialEq, Eq, Contiguous, Copy, Clone)]
//! # */
//! pub enum Color {
//!     Red = 0,
//!     Blue,
//!     Green,
//!     White,
//!     Black,
//! }
//! # unsafe impl Contiguous for Color {
//! #   type Int = u8;
//! #
//! #   const MIN_VALUE: u8 = 0;
//! #   const MAX_VALUE: u8 = 4;
//! # }
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
//! # #[derive(Debug, PartialEq, Eq)]
//! # /*
//! #[derive(Debug, PartialEq, Eq, TransparentWrapper)]
//! # */
//! pub struct SliceWrapper<T>(pub [T]);
//!
//! # unsafe impl<T> TransparentWrapper<[T]> for SliceWrapper<T> {}
//! #
//! impl<T> SliceWrapper<T> {
//!     // Using `constmuck` allows safely defining this function as a `const fn`
//!     pub const fn new(reff: &[T]) -> &Self {
//!         constmuck::wrapper::wrap_ref!(reff)
//!     }
//! }
//!
//! impl SliceWrapper<u32> {
//!     pub const fn sum(&self) -> u64 {
//!         let mut sum = 0;
//!         konst::for_range!{i in 0..self.0.len() =>
//!             sum += self.0[i] as u64;
//!         }
//!         sum
//!     }
//!     pub const fn find_first_even(&self) -> Option<(usize, u32)> {
//!         konst::for_range!{i in 0..self.0.len() =>
//!             if self.0[i] % 2 == 0 {
//!                 return Some((i, self.0[i]));
//!             }
//!         }
//!         None
//!     }
//!     
//! }
//!
//!
//! ```
//!
//! # Additional checks
//!
//! Additional checks are enabled in debug builds,
//! all of which cause panics when it'd have otherwise been Undefined Behavior
//! (caused by unsound `unsafe impl`s or calling `unsafe` constructor functions),
//! which means that there is a bug in some unsafe code somewhere.
//!
//! The precise checks are left unspecified so that they can change at any time.
//!
//! These checks are disabled by default in release builds,
//! to enable them you can use this in your Cargo.toml:
//!
//! ```toml
//! [profile.release.package.constmuck]
//! debug-assertions = true
//! ```
//!
//! # Features
//!
//! These are the features of this crate:
//!
//! - `"derive"`(disabled by default):
//! Enables `bytemuck`'s `"derive"` feature and reexports its derives.
//!
//! - `"rust_latest_stable"`(disabled by default):
//! Enables all items and functionality that requires stable Rust versions after 1.56.0.
//! Currently doesn't enable any other feature.
//!
//! - `"rust_1_57"`(disabled by default, requires Rust 1.57.0):
//! Causes this crate to use the `const_panic` dependency,
//! to improve the quality of panic messages.
//!
//! # No-std support
//!
//! `constmuck` is `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `constmuck` requires Rust 1.64.0.
//!
//! You can use the `"rust_latest_stable"` crate feature to get
//! all items and functionality that requires stable Rust versions after 1.56.0.
//!
//! [`bytemuck`]: bytemuck
//! [`konst`]: https://docs.rs/konst/*/konst/index.html
//! [`contiguous`]: ./contiguous/index.html
//! [`wrapper`]: ./wrapper/index.html

#![no_std]
#![deny(unused_results)]
#![deny(clippy::missing_safety_doc)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(all(doctest, feature = "derive"))]
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
pub use bytemuck::{self, Contiguous, Pod, PodCastError, TransparentWrapper, Zeroable};

pub use crate::{
    pod::{cast, cast_ref_alt, try_cast, try_cast_ref_alt},
    slice_fns::{bytes_of, cast_slice_alt, try_cast_slice_alt},
    zeroable::zeroed,
};

#[doc(hidden)]
pub mod __ {
    pub use core::mem::size_of;
    pub use core::ops::Range;
}

use constmuck_internal::const_panic;
