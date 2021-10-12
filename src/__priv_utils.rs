#![allow(missing_debug_implementations)]

use core::mem::ManuallyDrop;

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
