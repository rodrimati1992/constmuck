use core::{
    marker::PhantomData,
    mem::{self, ManuallyDrop},
};

use crate::{ImplsPod, Infer};

#[macro_export]
macro_rules! type_size {
    ($ty:ty) => {
        $crate::TypeSize::<_, $ty, { $crate::__::size_of::<$ty>() }>::__NEW__
    };
}

pub struct TypeSize<B, T, const SIZE: usize> {
    bounds: ManuallyDrop<B>,
    _private: PhantomData<T>,
}

impl<B: Copy, T, const SIZE: usize> Copy for TypeSize<B, T, SIZE> {}

impl<B: Copy, T, const SIZE: usize> Clone for TypeSize<B, T, SIZE> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Infer, T, const SIZE: usize> TypeSize<B, T, SIZE> {
    #[doc(hidden)]
    pub const __NEW__: Self = {
        if mem::size_of::<T>() != SIZE {
            [/* WTF */][mem::size_of::<T>()]
        } else {
            Self {
                bounds: ManuallyDrop::new(Infer::INFER),
                _private: PhantomData,
            }
        }
    };
}

impl<B, T, const SIZE: usize> TypeSize<B, T, SIZE> {
    pub const fn into_bounds(self) -> B {
        ManuallyDrop::into_inner(self.bounds)
    }
}
