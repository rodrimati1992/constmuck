#![no_std]

#[macro_use]
mod macros;

mod bytes_fns;

mod infer;

mod pod;

mod type_size;

#[doc(hidden)]
pub mod __priv_utils;

pub use bytemuck::{self, Pod, PodCastError};

pub use crate::{
    bytes_fns::bytes_of,
    infer::Infer,
    pod::{cast, cast_ref_alt, try_cast, try_cast_ref_alt, ImplsPod},
    type_size::TypeSize,
};

#[doc(hidden)]
pub mod __ {
    pub use core::mem::size_of;
}
