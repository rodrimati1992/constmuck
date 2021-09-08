use core::mem::{self, ManuallyDrop};

union Transmuter<F, T> {
    from: ManuallyDrop<F>,
    to: ManuallyDrop<T>,
}

pub(crate) const unsafe fn transmute<T, U>(from: T) -> U {
    let same_size = mem::size_of::<T>() == mem::size_of::<U>();
    [(/* expected T and U of the same size */)][(!same_size) as usize];
    ManuallyDrop::into_inner(
        Transmuter {
            from: ManuallyDrop::new(from),
        }
        .to,
    )
}
