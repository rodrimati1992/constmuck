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
        $ArgsTy:ident, 
        $reff:expr, 
        $( $Outer:ty, [$Inner:ty, $($__0:tt)*] )?
    ) => {
        // using a match so that this macro can be used with references to temporaries
        match $reff {
            inner => unsafe {
                let inner: &_ = inner;
                $crate::$ArgsTy $(::<$Outer, $Inner>)? {
                    reff: inner,
                    ptr: inner as *const _ as _,
                }.__call()
            }
        }
    }
}

#[macro_export]
macro_rules! wrapper_wrap_ref {
    ($reff:expr $(, $Outer:ty $(, $Inner:ty)?)? $(,)?) => {
        $crate::wrapper_inner!(
            __WrapRefArgs,
            $reff, 
            $($Outer, [$($Inner,)? _,])?
        )
    }
}

#[macro_export]
macro_rules! wrapper_peel_ref {
    ($reff:expr $(, $Outer:ty $(, $Inner:ty)?)? $(,)?) => {
        $crate::wrapper_inner!(
            __PeelRefArgs,
            $reff, 
            $($Outer, [$($Inner,)? _,])?
        )
    };
}

///////////////////////////

#[doc(hidden)]
pub struct __PeelRefArgs<'a, Outer: ?Sized, Inner: ?Sized> {
    pub reff: &'a Outer,
    pub ptr: *const Inner,
}

impl<'a, Outer: ?Sized, Inner: ?Sized> __PeelRefArgs<'a, Outer, Inner> {
    /// # Safety
    /// 
    /// `ptr` must be `_reff as *const Outer as *const Inner`
    #[track_caller]
    #[doc(hidden)]
    pub const unsafe fn __call(self) -> &'a Inner
    where
        Outer: TransparentWrapper<Inner>
    {
        #[cfg(feature = "debug_checks")]
        if TWHelper::<Outer, Inner>::NOT_SAME_SIZE {
            unequal_ptr_size_panic(
                core::mem::size_of::<*const Outer>(),
                core::mem::size_of::<*const Inner>(),
            )
        }

        &*self.ptr
    }
}


#[doc(hidden)]
pub struct __WrapRefArgs<'a, Outer: ?Sized, Inner: ?Sized> {
    pub reff: &'a Inner,
    pub ptr: *const Outer,
}

impl<'a, Outer: ?Sized, Inner: ?Sized> __WrapRefArgs<'a, Outer, Inner> {
    /// # Safety
    /// 
    /// `ptr` must be `_reff as *const Outer as *const Inner`
    #[track_caller]
    #[doc(hidden)]
    pub const unsafe fn __call(self) -> &'a Outer
    where
        Outer: TransparentWrapper<Inner>
    {
        #[cfg(feature = "debug_checks")]
        if TWHelper::<Outer, Inner>::NOT_SAME_SIZE {
            unequal_ptr_size_panic(
                core::mem::size_of::<*const Inner>(),
                core::mem::size_of::<*const Outer>(),
            )
        }

        &*self.ptr
    }
}

///////////////////////////

#[cfg(feature = "debug_checks")]
struct TWHelper<Outer: ?Sized, Inner: ?Sized>(PhantomData<Outer>, PhantomData<Inner>);


#[cfg(feature = "debug_checks")]
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
#[cfg(feature = "debug_checks")]
const fn unequal_ptr_size_panic(size_of_from: usize, size_of_to: usize) -> ! {
    use crate::const_panic::{FmtArg as FA, PanicVal as PV};

    crate::const_panic::concat_panic(&[&[
        PV::write_str("\nexpected transmute not to change the pointer size,"),
        PV::write_str(" size goes from: "), PV::from_usize(size_of_from, FA::DEBUG),
        PV::write_str(" to: "), PV::from_usize(size_of_to, FA::DEBUG),
    ]])
}



