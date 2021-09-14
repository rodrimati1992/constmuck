#![no_std]

#[doc(hidden)]
pub mod trans;

#[doc(hidden)]
pub mod proof_types;

#[cfg(feature = "debug_checks")]
pub use crate::trans::NotSameSize;

#[doc(hidden)]
pub use crate::{
    trans::{
        AssertTP,
        AssertTPCasted,
        AssertTWPInner,
        AssertTWPOuter,
        PhantomRef,
        TPPtrToRef,
    },
    proof_types::{TransparentWrapperProof, TransmutableProof},
};