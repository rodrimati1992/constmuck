#![no_std]

#[macro_use]
mod macros;

mod infer;

mod pod;

mod __priv_utils;

pub use bytemuck::{self, Pod, PodCastError};

pub use crate::{
    infer::Infer,
    pod::{cast, try_cast, ImplsPod},
};
