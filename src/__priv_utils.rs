use core::mem::ManuallyDrop;

#[repr(C)]
pub(crate) union Transmuter<F, T> {
    pub(crate) from: ManuallyDrop<F>,
    pub(crate) to: ManuallyDrop<T>,
}

#[repr(C)]
pub(crate) union PtrToRef<'a, P> {
    pub(crate) ptr: *const P,
    pub(crate) reff: &'a P,
}
