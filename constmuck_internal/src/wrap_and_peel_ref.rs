//! IsTransparentWrapper-related transmutation between ?Sized references 
//! 

use core::marker::PhantomData;

///////////////////////////////////////////////////////

#[macro_export]
macro_rules! wrapper_inner {
    ($reff:expr, $is_tw:expr, $panic:ident, $XRef:ident) => {
        // using a match so that this macro can be used with references to temporaries
        match ($reff, $is_tw) {
            (inner, is_wrapper) => {
                let ass: $crate::$XRef<_, _> = (
                    inner,
                    (
                        is_wrapper._transparent_wrapper_proof,
                        $crate::TWCastLifetimes::NEW,
                    )
                );

                $crate::__check_size!{ass.1.0, $panic}

                $crate::CastedWrapperPtrToRef{
                    // see comment on CastedWrapperPtr struct for why this is casted twice
                    ptr: $crate::CastedWrapperPtr(ass.0 as *const _ as *const _, ass.1)
                }.reff
            }
        }
    }
}


#[macro_export]
macro_rules! wrapper_wrap_ref {
    ($reff:expr, $is_tw:expr $(,)*) => {
        unsafe{ $crate::wrapper_inner!($reff, $is_tw, panic_wrap, FromInnerToOuterRef) }
    };
}

#[macro_export]
macro_rules! wrapper_peel_ref {
    ($reff:expr, $is_tw:expr $(,)*) => {
        unsafe{ $crate::wrapper_inner!($reff, $is_tw, panic_peel, FromOuterToInnerRef) }
    };
}

///////////////////////////

// Represents an `Outer: TransparentWrapper<Inner>` bound
// and what direction between &Outer and &Inner the transmute goes
// 
// Fro and To represent the direction the conversion goes and are either of these:
// - in `wrapper_peel_ref` macro: Fro = Outer, To = Inner
// - in `wrapper_wrap_ref` macro: Fro = Inner, To = Outer
pub type TWProofAndLifetime<'a, Fro, To, Outer, Inner> = (
    crate::TransparentWrapperProof<Outer, Inner>,
    TWCastLifetimes<'a, Fro, To>,
);

// Unifies the lifetimes of references to Fro and To,
// as well as make the lifetimes for both types invariant.
pub struct TWCastLifetimes<'a, Fro: ?Sized, To: ?Sized>(
    PhantomData<(
        fn(&'a Fro) -> &'a Fro,
        fn(&'a To) -> &'a To,
    )>
);

impl<'a, F: ?Sized + 'a, T: ?Sized + 'a> TWCastLifetimes<'a, F, T> {
    pub const NEW: Self = Self(PhantomData);
}

// Used for `&'a Inner` to `&'a Outer` transmutes 
// where `Outer` implements `TransparentWrapper<Inner>`
pub type FromInnerToOuterRef<'a, Outer, Inner> = (
    &'a Inner,
    TWProofAndLifetime<'a, Inner, Outer, Outer, Inner>,
);

// Used for `&'a Outer` to `&'a Inner` transmutes
// where `Outer` implements `TransparentWrapper<Inner>`
pub type FromOuterToInnerRef<'a, Outer, Inner> = (
    &'a Outer,
    TWProofAndLifetime<'a, Outer, Inner, Outer, Inner>,
);

// Used for `&'a Fro` to `&'a To` transmutes
//
// The way that the `wrapper_inner` macro is invoked in this module guarantees
// that either of these bounds apply:
// - `Fro: TransparentWrapper<To>`
// - `To: TransparentWrapper<Fro>`
// 
// `&'a Fro` is casted to `*const Fro` then `*const To` in the `wrapper_inner` macro
// to not rely on the layout of pointers to !Sized types being the same.
//
// There's no guarantee that pointers to !Sized types are laid out like 
// ```
// #[repr(C)]
// struct Pointer<Metadata>{
//      data: *const (),
//      metadata: Metadata,
// }
// ```
// both fields could easily be swapped in the process of doing `*const Fro as *const To`.
// That's why the `constmuck::wrapper::{peel_ref, wrap_ref}` functions take 
// references to `Sized` types(the layout of those pointers is guaranteed),
// and the `wrapper_inner` macro uses pointer `as` casts.
#[repr(transparent)]
pub struct CastedWrapperPtr<'a, Fro: ?Sized, To: ?Sized, Outer: ?Sized, Inner: ?Sized>(
    pub *const To,
    pub TWProofAndLifetime<'a, Fro, To, Outer, Inner>,
);

#[repr(C)]
pub union CastedWrapperPtrToRef<'a, Fro: ?Sized, To: ?Sized, Outer: ?Sized, Inner: ?Sized> {
    pub ptr: CastedWrapperPtr<'a, Fro, To, Outer, Inner>,
    pub reff: &'a To,
}

macro_rules! impl_copy {
    ($type:ident[$($typ:ident),*]) => (
        impl<'a, $($typ: ?Sized),*>  Copy  for $type<'a, $($typ),*> {}

        impl<'a, $($typ: ?Sized),*>  Clone for $type<'a, $($typ),*> {
            fn clone(&self) -> Self {
                *self
            }
        }
    )
}

impl_copy!{CastedWrapperPtr[Fro, To, Outer, Inner]}
impl_copy!{TWCastLifetimes[Fro, To]}
