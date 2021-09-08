use core::mem::ManuallyDrop;

#[repr(C)]
pub(crate) union Transmuter<F, T> {
    pub(crate) from: ManuallyDrop<F>,
    pub(crate) to: ManuallyDrop<T>,
}
