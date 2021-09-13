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


#[macro_export]
macro_rules! wrapper_inner {
    ($inner:expr, $impls_wrapper:expr, $AssertTWP:ident, $tw_field:ident) => {
        match ($inner, $impls_wrapper) {
            (inner, impls_wrapper) => {
                let ass = $crate::$AssertTWP(
                    inner,
                    impls_wrapper._transparent_wrapper_proof,
                    $crate::PhantomRef::NEW,
                );

                unsafe{
                    $crate::TPPtrToRef{
                        ptr: $crate::AssertTPCasted(
                            ass.0 as *const _ as *const _,
                            ass.1.$tw_field,
                            ass.2,
                        ),
                    }.reff
                }
            }
        }
    }
}


#[macro_export]
macro_rules! wrapper_wrap_ref {
    ($inner:expr, $impls_wrapper:expr $(,)*) => {
        $crate::wrapper_inner!($inner, $impls_wrapper, AssertTWPInner, from_inner)
    };
}

#[macro_export]
macro_rules! wrapper_peel_ref {
    ($outer:expr, $impls_wrapper:expr $(,)*) => {
        $crate::wrapper_inner!($outer, $impls_wrapper, AssertTWPOuter, into_inner)
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

///////////////////////////


// AssertTransparentWrapperProof
#[repr(transparent)]
pub struct AssertTWPOuter<'a, Outer: ?Sized, Inner: ?Sized>(
    pub &'a Outer,
    pub crate::TransparentWrapperProof<Outer, Inner>,
    pub PhantomRef<'a, Outer>,
);

///////////////////////////

// AssertTransparentWrapperProof
#[repr(transparent)]
pub struct AssertTWPInner<'a, Outer: ?Sized, Inner: ?Sized>(
    pub &'a Inner,
    pub crate::TransparentWrapperProof<Outer, Inner>,
    pub PhantomRef<'a, Inner>,
);

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


