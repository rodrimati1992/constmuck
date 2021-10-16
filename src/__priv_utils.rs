#![allow(missing_debug_implementations)]

use core::{
    marker::PhantomData,
    mem::{size_of, ManuallyDrop},
};

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

// For dereferencing raw pointers,
// since `&*raw_ptr` doesn't work in const contexts yet.
#[repr(C)]
pub(crate) union PtrToRef<'a, P: ?Sized> {
    pub(crate) ptr: *const P,
    pub(crate) reff: &'a P,
}

#[repr(C)]
pub union ManuallyDropAsInner<'a, T> {
    pub(crate) outer: &'a ManuallyDrop<T>,
    pub(crate) inner: &'a T,
}

pub(crate) const fn manuallydrop_as_inner<T>(outer: &ManuallyDrop<T>) -> &T {
    unsafe { ManuallyDropAsInner { outer }.inner }
}

// checking that the size of an array is just `size_of::<T>() * LEN`
//
pub(crate) struct SizeIsStride<T, const LEN: usize>(PhantomData<fn() -> T>);

impl<T, const LEN: usize> SizeIsStride<T, LEN> {
    pub(crate) const V: bool = { size_of::<[T; LEN]>() != size_of::<T>() * LEN };

    #[cold]
    #[inline(never)]
    #[allow(unconditional_panic)]
    pub(crate) const fn panic() -> ! {
        let x = 0;
        [/* uh oh, size != stride */][x]
    }
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
