#![no_std]
//! Implementation detail of constmuck,
//! this crate is allowed to make breaking changing at any point.

#[doc(hidden)]
pub use core::marker::PhantomData;

#[doc(hidden)]
pub use const_panic;

use bytemuck::TransparentWrapper;


///////////////////////////////////////////////////////

#[macro_export]
macro_rules! wrapper_inner {
    (
        $function:ident, 
        $reff:expr, 
        $( $Outer:ty, [$Inner:ty, $($__0:tt)*] )?
    ) => {
        // using a match so that this macro can be used with references to temporaries
        match $reff {
            inner => unsafe {
                let inner: &_ = inner;
                let ptr: *const _ = inner;
                $crate::$function $(::<$Outer, $Inner>)? (inner, ptr as *const _)
            }
        }
    }
}

#[macro_export]
macro_rules! wrapper_wrap_ref {
    ($reff:expr $(, $Outer:ty $(, $Inner:ty)?)? $(,)?) => {
        $crate::wrapper_inner!(
            __wrap_ref_helper,
            $reff, 
            $($Outer, [$($Inner,)? _,])?
        )
    }
}

#[macro_export]
macro_rules! wrapper_peel_ref {
    ($reff:expr $(, $Outer:ty $(, $Inner:ty)?)? $(,)?) => {
        $crate::wrapper_inner!(
            __peel_ref_helper,
            $reff, 
            $($Outer, [$($Inner,)? _,])?
        )
    };
}

///////////////////////////

/// # Safety
/// 
/// `ptr` must be `_reff as *const Outer as *const Inner`
#[doc(hidden)]
#[track_caller]
pub const unsafe fn __peel_ref_helper<'a, Outer: ?Sized, Inner: ?Sized>(
    _reff: &'a Outer,
    ptr: *const Inner,
) -> &'a Inner
where
    Outer: TransparentWrapper<Inner>
{
    #[cfg(debug_assertions)]
    if TWHelper::<Outer, Inner>::NOT_SAME_SIZE {
        unequal_ptr_size_panic(
            core::mem::size_of::<*const Outer>(),
            core::mem::size_of::<*const Inner>(),
        )
    }

    &*ptr
}


/// # Safety
/// 
/// `ptr` must be `_reff as *const Outer as *const Inner`
#[doc(hidden)]
#[track_caller]
pub const unsafe fn __wrap_ref_helper<'a, Outer: ?Sized, Inner: ?Sized>(
    _reff: &'a Inner,
    ptr: *const Outer,
) -> &'a Outer
where
    Outer: TransparentWrapper<Inner>
{
    #[cfg(debug_assertions)]
    if TWHelper::<Outer, Inner>::NOT_SAME_SIZE {
        unequal_ptr_size_panic(
            core::mem::size_of::<*const Inner>(),
            core::mem::size_of::<*const Outer>(),
        )
    }

    &*ptr
}

///////////////////////////

#[cfg(debug_assertions)]
struct TWHelper<Outer: ?Sized, Inner: ?Sized>(PhantomData<Outer>, PhantomData<Inner>);


#[cfg(debug_assertions)]
impl<Outer: ?Sized, Inner: ?Sized> TWHelper<Outer, Inner> 
where
    Outer: TransparentWrapper<Inner>
{
    const NOT_SAME_SIZE: bool =
        core::mem::size_of::<*const Outer>() != core::mem::size_of::<*const Inner>();
}


#[track_caller]
#[cold]
#[inline(never)]
const fn unequal_ptr_size_panic(size_of_from: usize, size_of_to: usize) -> ! {
    use crate::const_panic::{FmtArg as FA, PanicVal as PV};

    crate::const_panic::concat_panic(&[&[
        PV::write_str("\nexpected transmute not to change the pointer size,"),
        PV::write_str(" size goes from: "), PV::from_usize(size_of_from, FA::DEBUG),
        PV::write_str(" to: "), PV::from_usize(size_of_to, FA::DEBUG),
    ]])
}



