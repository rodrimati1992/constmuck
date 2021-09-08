#![no_std]

#[macro_use]
mod macros;

mod infer;

mod pod;

mod utils;

pub use bytemuck;

pub use crate::{
    infer::Infer,
    pod::{cast, ImplsPod},
};
