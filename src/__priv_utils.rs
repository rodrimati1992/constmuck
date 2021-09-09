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
pub union BytesAndVal<T, const N: usize> {
    pub(crate) bytes: [u8; N],
    pub(crate) value: ManuallyDrop<T>,
}
