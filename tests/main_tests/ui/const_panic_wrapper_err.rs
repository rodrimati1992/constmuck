#[cfg(feature = "debug_checks")]
const _: () = {
    use constmuck::wrapper::{
        TransparentWrapper, 
        peel, peel_ref, peel_slice,
        wrap, wrap_ref, wrap_slice,
    };

    // for testing that `feature = "debug_checks"` rejects 
    // peeling from or wrapping into this type,
    // which is 0-sized, but with the same alignment as `T`.
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct UnitW<T>([T; 0]);

    unsafe impl<T> TransparentWrapper<T> for UnitW<T> {}

    //

    #[derive(Copy)]
    #[repr(packed)]
    pub struct Pack<T>(pub T);

    impl<T: Copy> Clone for Pack<T> {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }

    unsafe impl<T> TransparentWrapper<T> for Pack<T> {}

    const _: () = { peel(Pack(0u16)); };

    const _: () = { peel(UnitW::<u16>([])); };

    const _: () = { peel_ref(&Pack(0u16)); };

    const _: () = { peel_ref(&UnitW::<u16>([])); };

    const _: () = { peel_slice(&[Pack(0u16)]); };

    const _: () = { peel_slice(&[UnitW::<u16>([])]); };

    const _: () = { wrap::<Pack<_>, _>(0u16); };

    const _: () = { wrap::<UnitW<_>, _>(0u16); };

    const _: () = { wrap_ref::<Pack<_>, _>(&0u16); };

    const _: () = { wrap_ref::<UnitW<_>, _>(&0u16); };

    const _: () = { wrap_slice::<Pack<_>, _>(&[0u16]); };

    const _: () = { wrap_slice::<UnitW<_>, _>(&[0u16]); };


};

fn main(){}