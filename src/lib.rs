#![no_std]

#[macro_use]
mod macros;

pub mod copying;

mod infer;

mod pod;

mod slice_fns;

mod type_size;

pub mod transmutable;

mod zeroable;

#[doc(hidden)]
pub mod __priv_utils;

#[doc(no_inline)]
pub use bytemuck::{self, Pod, PodCastError};

pub use crate::{
    copying::impls_copy::ImplsCopy,
    infer::Infer,
    pod::{cast, cast_ref_alt, try_cast, try_cast_ref_alt, ImplsPod},
    slice_fns::{bytes_of, cast_slice_alt, try_cast_slice_alt},
    transmutable::transmutable_into::TransmutableInto,
    type_size::TypeSize,
    zeroable::{zeroed, zeroed_array, ImplsZeroable},
};

#[doc(hidden)]
pub mod __ {
    pub use core::mem::size_of;
}
