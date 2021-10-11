#![no_std]

#[doc(hidden)]
pub mod trans;

#[doc(hidden)]
pub mod proof_types;

#[cfg(feature = "debug_checks")]
pub use crate::trans::CheckSameSize;

#[doc(hidden)]
pub use crate::{
    trans::{
        AssertTPCasted,
        AssertTWPInner,
        AssertTWPOuter,
        PhantomRef,
        TPPtrToRef,
    },
    proof_types::{SameReprProof, TransparentWrapperProof},
};