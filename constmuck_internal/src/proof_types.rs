use core::marker::PhantomData;

// proof that `Outer` implements `bytemuck::TransparentWrapper<Inner>`
pub struct TransparentWrapperProof<Outer: ?Sized, Inner: ?Sized>{
    _priv: PhantomData<(
        // Makes this invariant over the lifetimes in `Outer` and `Inner`,
        // so that the type parameters can't be coerced to a different lifetime.
        fn(PhantomData<Outer>) -> PhantomData<Outer>,
        fn(PhantomData<Inner>) -> PhantomData<Inner>,
    )>
}

impl<Outer: ?Sized, Inner: ?Sized> TransparentWrapperProof<Outer, Inner> {
    const __NEW: Self = Self{_priv: PhantomData};
    #[inline(always)]
    pub const unsafe fn new_unchecked() -> Self {
        Self::__NEW
    }
}

impl<A: ?Sized, B: ?Sized> Copy for TransparentWrapperProof<A, B> {}

impl<A: ?Sized, B: ?Sized> Clone for TransparentWrapperProof<A, B> {
    fn clone(&self) -> Self {
        *self
    }
}


#[doc(hidden)]
#[cfg(feature = "debug_checks")]
#[macro_export]
macro_rules! __check_size {
    ($transparent_wrapper_proof:expr) => (
        if $crate::TransparentWrapperProof::is_not_same_size($transparent_wrapper_proof) {
            let x = 0;
            let _: () = [/* expected transmute not to change the pointer size */][x];
            loop{}
        }
    )
}

#[cfg(not(feature = "debug_checks"))]
#[macro_export]
macro_rules! __check_size {
    ($transparent_wrapper_proof:expr) => ()
}

#[cfg(feature = "debug_checks")]
#[doc(hidden)]
impl<Outer: ?Sized, Inner: ?Sized> TransparentWrapperProof<Outer, Inner> {
    const NOT_SAME_SIZE: bool =
        core::mem::size_of::<*const Outer>() != core::mem::size_of::<*const Inner>();

    #[inline(always)]
    pub const fn is_not_same_size(self) -> bool {
        Self::NOT_SAME_SIZE
    }
}

