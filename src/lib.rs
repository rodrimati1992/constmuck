//! Const equivalents of many [`bytemuck`] functions,
//! and a few additional const functions.
//!
//! The `*_alt` functions aren't exactly equivalent to the `bytemuck` ones,
//! each one describes how it's different.
//!
//! # Features
//!
//! There are no crate features to enable in the `Cargo.toml` yet.
//!
//!

#![no_std]

#[macro_use]
mod macros;

pub mod copying;

pub mod contiguous;

mod infer;

mod pod;

mod slice_fns;

mod type_size;

pub mod transmutable;

pub mod wrapper;

mod zeroable;

#[doc(hidden)]
pub mod __priv_utils;

#[doc(no_inline)]
pub use bytemuck::{self, Contiguous, Pod, PodCastError, TransparentWrapper, Zeroable};

pub use crate::{
    contiguous::impls_contiguous::ImplsContiguous,
    copying::impls_copy::ImplsCopy,
    infer::Infer,
    pod::{cast, cast_ref_alt, try_cast, try_cast_ref_alt, ImplsPod},
    slice_fns::{bytes_of, cast_slice_alt, try_cast_slice_alt},
    transmutable::transmutable_into::TransmutableInto,
    type_size::TypeSize,
    wrapper::impls_tw::ImplsTransparentWrapper,
    zeroable::{zeroed, zeroed_array, ImplsZeroable},
};

#[doc(hidden)]
pub mod __ {
    pub use core::mem::size_of;
}
