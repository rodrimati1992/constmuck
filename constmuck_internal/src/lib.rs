#![no_std]
//! Implementation detail of constmuck,
//! this crate is allowed to make breaking changing at any point.

mod wrap_and_peel_ref;

mod proof_types;

#[doc(hidden)]
pub use core::marker::PhantomData;

#[doc(hidden)]
pub use crate::{
    wrap_and_peel_ref::{
        TWCastLifetimes,
        FromInnerToOuterRef,
        FromOuterToInnerRef,
        CastedWrapperPtr,
        CastedWrapperPtrToRef,
    },
    proof_types::TransparentWrapperProof,
};