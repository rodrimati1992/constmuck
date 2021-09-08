#![no_std]

#[macro_use]
mod macros;

mod infer;

mod pod;

mod __priv_utils;

pub use bytemuck::{self, Pod, PodCastError};

pub use crate::{
    infer::Infer,
    pod::{cast, cast_ref, try_cast, try_cast_ref, ImplsPod},
};
