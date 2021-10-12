use core::marker::PhantomData;

#[non_exhaustive]
pub struct TransparentWrapperProof<Outer: ?Sized, Inner: ?Sized>{
    pub from_inner: SameReprProof<Inner, Outer>,
    pub into_inner: SameReprProof<Outer, Inner>,
}

impl<Outer: ?Sized, Inner: ?Sized> TransparentWrapperProof<Outer, Inner> {
    const __NEW: Self = unsafe {
        Self{
            from_inner: SameReprProof::new_unchecked(),
            into_inner: SameReprProof::new_unchecked(),
        }
    };

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




pub struct SameReprProof<Fro: ?Sized, To: ?Sized>{
    _priv: PhantomData<(
        // Makes this invariant over the lifetimes in `Fro` and `To`
        // so that it's not possible to change lifetime parameters.
        fn(PhantomData<Fro>) -> PhantomData<Fro>,
        fn(PhantomData<To>) -> PhantomData<To>,
    )>,
}

impl<Fro: ?Sized, To: ?Sized> SameReprProof<Fro, To> {
    const __NEW: Self = Self{_priv: PhantomData};
    pub const unsafe fn new_unchecked() -> Self {
        Self::__NEW
    }
}

impl<A: ?Sized, B: ?Sized> Copy for SameReprProof<A, B> {}

impl<A: ?Sized, B: ?Sized> Clone for SameReprProof<A, B> {
    fn clone(&self) -> Self {
        *self
    }
}
