#![allow(missing_debug_implementations)]

use core::mem::ManuallyDrop;

use crate::const_panic::{FmtArg as FA, PanicVal as PV};

// allows transmuting between arbitrary Sized types.
#[repr(C)]
pub(crate) union Transmuter<F, T> {
    pub(crate) from: ManuallyDrop<F>,
    pub(crate) to: ManuallyDrop<T>,
}

#[repr(C)]
pub(crate) union TransmuterFromCopy<F: Copy, T> {
    pub(crate) from: F,
    pub(crate) to: ManuallyDrop<T>,
}

#[repr(C)]
pub union ManuallyDropAsInner<'a, T> {
    pub(crate) outer: &'a ManuallyDrop<T>,
    pub(crate) inner: &'a T,
}

pub(crate) const fn manuallydrop_as_inner<T>(outer: &ManuallyDrop<T>) -> &T {
    unsafe { ManuallyDropAsInner { outer }.inner }
}

#[cold]
#[inline(never)]
pub(crate) const fn transmute_unequal_size_panic(size_of_t: usize, size_of_u: usize) -> ! {
    crate::const_panic::concat_panic(&[&[
        PV::write_str("\nexpected transmute not to change the size,"),
        PV::write_str(" size goes from: "),
        PV::from_usize(size_of_t, FA::DEBUG),
        PV::write_str(" to: "),
        PV::from_usize(size_of_u, FA::DEBUG),
    ]])
}

#[cold]
#[inline(never)]
pub(crate) const fn transmute_unequal_align_panic(align_of_t: usize, align_of_u: usize) -> ! {
    crate::const_panic::concat_panic(&[&[
        PV::write_str("\nexpected transmute not to change alignment,"),
        PV::write_str(" alignment goes from: "),
        PV::from_usize(align_of_t, FA::DEBUG),
        PV::write_str(" to: "),
        PV::from_usize(align_of_u, FA::DEBUG),
    ]])
}

#[cold]
#[inline(never)]
pub(crate) const fn unequal_size_panic(size_of_t: usize, size_of_u: usize) -> ! {
    crate::const_panic::concat_panic(&[&[
        PV::write_str("\nthe size of T and U is not the same"),
        PV::write_str("\nsize_of::<T>(): "),
        PV::from_usize(size_of_t, FA::DEBUG),
        PV::write_str("\nsize_of::<U>(): "),
        PV::from_usize(size_of_u, FA::DEBUG),
    ]])
}
#[cold]
#[inline(never)]
pub(crate) const fn incompatible_alignment_panic(align_of_t: usize, align_of_u: usize) -> ! {
    crate::const_panic::concat_panic(&[&[
        PV::write_str("\nThe alignment of T is lower than U"),
        PV::write_str("\nalign_of::<T>(): "),
        PV::from_usize(align_of_t, FA::DEBUG),
        PV::write_str("\nalign_of::<U>(): "),
        PV::from_usize(align_of_u, FA::DEBUG),
    ]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manuallydrop_as_inner_test() {
        macro_rules! case {
            ($value:expr) => {
                assert_eq!(manuallydrop_as_inner(&ManuallyDrop::new($value)), &$value);
            };
        }

        case!("hello");
        case!(100);
        case!(true);
        case!(false);
    }
}
