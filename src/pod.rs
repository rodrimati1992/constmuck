use core::marker::PhantomData;

use bytemuck::Pod;

mod __ {
    use super::*;

    pub struct ImplsPod<T> {
        _private: PhantomData<fn() -> T>,
    }

    impl<T> Copy for ImplsPod<T> {}

    impl<T> Clone for ImplsPod<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: Pod> ImplsPod<T> {
        pub const NEW: Self = Self {
            _private: PhantomData,
        };
    }
}
pub use __::ImplsPod;

impl<T: Pod> crate::Infer for ImplsPod<T> {
    const INFER: Self = Self::NEW;
}

pub const fn cast<T, U>(from: T, _bounds: (ImplsPod<T>, ImplsPod<U>)) -> U {
    unsafe { crate::utils::transmute(from) }
}
