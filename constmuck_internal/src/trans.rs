//! trans for transmutation

use core::marker::PhantomData;


#[cfg(feature = "debug_checks")]
#[macro_export]
macro_rules! __check_size {
    ($transmutable_into:expr) => (
        if let $crate::CheckSameSize{same_size: false, proof, ..} =
            $crate::CheckSameSize::NEW 
        {
            [proof, $transmutable_into];
            let x = 0;
            let _: () = [/* expected transmute not to change the pointer size */][x];
            loop{}
        }
    )
}

#[cfg(not(feature = "debug_checks"))]
#[macro_export]
macro_rules! __check_size {
    ($transmutable_into:expr) => ()
}


#[cfg(feature = "debug_checks")]
#[repr(transparent)]
#[non_exhaustive]
pub struct CheckSameSize<L: ?Sized, R: ?Sized>{
    pub same_size: bool,

    pub proof: crate::SameReprProof<L, R>,
}

#[cfg(feature = "debug_checks")]
impl<L: ?Sized, R: ?Sized> CheckSameSize<L, R> {
    pub const NEW: Self = Self {
        same_size: core::mem::size_of::<*const L>() == core::mem::size_of::<*const R>(),
        proof: unsafe{ crate::SameReprProof::new_unchecked() },
    };
}

///////////////////////////////////////////////////////

#[macro_export]
macro_rules! wrapper_inner {
    ($inner:expr, $is_wrapper:expr, $AssertTWP:ident, $tw_field:ident) => {
        match ($inner, $is_wrapper) {
            (inner, is_wrapper) => {
                let ass = $crate::$AssertTWP(
                    inner,
                    is_wrapper._transparent_wrapper_proof,
                    $crate::PhantomRef::NEW,
                );

                $crate::__check_size!{ass.1.$tw_field}

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
    ($inner:expr, $is_wrapper:expr $(,)*) => {
        $crate::wrapper_inner!($inner, $is_wrapper, AssertTWPInner, from_inner)
    };
}

#[macro_export]
macro_rules! wrapper_peel_ref {
    ($outer:expr, $is_wrapper:expr $(,)*) => {
        $crate::wrapper_inner!($outer, $is_wrapper, AssertTWPOuter, into_inner)
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
    pub crate::SameReprProof<Fro, To>,
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


