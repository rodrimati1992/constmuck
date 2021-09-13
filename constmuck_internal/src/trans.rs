//! trans for transmutation

use core::marker::PhantomData;

#[macro_export]
macro_rules! transmute_ref {
    ($reference:expr, $transmutable_into:expr $(,)*) => {
        match ($reference, $transmutable_into) {
            (reference, transmutable_into) => {
                let ass = $crate::AssertTP(
                    reference,
                    transmutable_into._transmutable_into_proof,
                    $crate::PhantomRef::NEW,
                );

                unsafe{
                    $crate::TPPtrToRef{
                        ptr: $crate::AssertTPCasted(ass.0 as *const _ as *const _, ass.1, ass.2),
                    }.reff
                }
            }
        }
    };
}


///////////////////////////

pub struct PhantomRef<'a, T: ?Sized>{
    _priv: PhantomData<fn(&'a T) -> &'a T>,
}

impl<'a, T: 'a + ?Sized> PhantomRef<'a, T> {
    pub const NEW: Self = Self{_priv: PhantomData};
}

impl<'a, T: 'a + ?Sized> Copy for PhantomRef<'a, T> {}

impl<'a, T: 'a + ?Sized> Clone for PhantomRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

///////////////////////////


// AssertTransmutableProof
#[repr(transparent)]
pub struct AssertTP<'a, Fro: ?Sized, To: ?Sized>(
    pub &'a Fro,
    pub crate::TransmutableProof<Fro, To>,
    pub PhantomRef<'a, Fro>,
);

impl<'a, Fro: ?Sized, To: ?Sized> Copy for AssertTP<'a, Fro, To> {}

impl<'a, Fro: ?Sized, To: ?Sized> Clone for AssertTP<'a, Fro, To> {
    fn clone(&self) -> Self {
        *self
    }
}

///////////////////////////

#[repr(transparent)]
pub struct AssertTPCasted<'a, Fro: ?Sized, To: ?Sized>(
    pub *const To,
    pub crate::TransmutableProof<Fro, To>,
    pub PhantomRef<'a, Fro>,
);

impl<'a, Fro: ?Sized, To: ?Sized> Copy for AssertTPCasted<'a, Fro, To> {}

impl<'a, Fro: ?Sized, To: ?Sized> Clone for AssertTPCasted<'a, Fro, To> {
    fn clone(&self) -> Self {
        *self
    }
}

///////////////////////////

#[repr(C)]
pub union TPPtrToRef<'a, T: ?Sized, P: ?Sized> {
    pub ptr: AssertTPCasted<'a, T, P>,
    pub reff: &'a P,
}


