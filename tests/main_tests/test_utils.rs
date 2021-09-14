use std::{
    any::Any,
    cmp::PartialEq,
    fmt::{self, Debug},
    panic::{catch_unwind, AssertUnwindSafe, Location},
};

use constmuck::{Pod, TransparentWrapper, Zeroable};

pub type ThreadError = Box<dyn Any + Send + 'static>;

#[derive(Debug, Clone)]
pub struct ShouldHavePanickedAt {
    pub span: &'static Location<'static>,
}

#[track_caller]
pub fn must_panic<F, R>(f: F) -> Result<ThreadError, ShouldHavePanickedAt>
where
    F: FnOnce() -> R,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(_) => Err(ShouldHavePanickedAt {
            span: Location::caller(),
        }),
        Err(e) => Ok(e),
    }
}

////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Wrap<T: ?Sized>(pub T);

unsafe impl<T: Pod> Pod for Wrap<T> {}

unsafe impl<T: Zeroable> Zeroable for Wrap<T> {}

unsafe impl<T: ?Sized> TransparentWrapper<T> for Wrap<T> {}

////////////////////////////////////////////////////////

#[derive(Copy)]
#[repr(packed)]
pub struct Pack<T>(pub T);

impl<T: Copy> Clone for Pack<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

unsafe impl<T: Zeroable> Zeroable for Pack<T> {}

unsafe impl<T: Pod> Pod for Pack<T> {}

impl<T> Debug for Pack<T>
where
    T: Copy + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as Debug>::fmt(&{ self.0 }, f)
    }
}

impl<T> PartialEq for Pack<T>
where
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        ({ self.0 }) == ({ other.0 })
    }
}
