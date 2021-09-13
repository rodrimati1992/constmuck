use core::marker::PhantomData;

pub struct TransparentWrapperProof<Outer: ?Sized, Inner: ?Sized>{
    _priv: PhantomData<(
        // Makes this invariant over the lifetimes in `Outer` and `Inner`
        // so that it's not possible to change lifetime parameters.
        fn(PhantomData<Outer>) -> PhantomData<Outer>,
        fn(PhantomData<Inner>) -> PhantomData<Inner>,
    )>
}

impl<Outer: ?Sized, Inner: ?Sized> TransparentWrapperProof<Outer, Inner> {
    const __NEW: Self = Self{_priv: PhantomData};
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




pub struct TransmutableProof<Fro: ?Sized, To: ?Sized>{
    _priv: PhantomData<(
        // Makes this invariant over the lifetimes in `Fro` and `To`
        // so that it's not possible to change lifetime parameters.
        fn(PhantomData<Fro>) -> PhantomData<Fro>,
        fn(PhantomData<To>) -> PhantomData<To>,
    )>,
}

impl<Fro: ?Sized, To: ?Sized> TransmutableProof<Fro, To> {
    const __NEW: Self = Self{_priv: PhantomData};
    pub const unsafe fn new_unchecked() -> Self {
        Self::__NEW
    }
}

impl<A: ?Sized, B: ?Sized> Copy for TransmutableProof<A, B> {}

impl<A: ?Sized, B: ?Sized> Clone for TransmutableProof<A, B> {
    fn clone(&self) -> Self {
        *self
    }
}
