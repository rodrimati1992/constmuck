#[doc(hidden)]
pub mod trans;

#[doc(hidden)]
pub mod proof_types;

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