use core::marker::PhantomData;

// proof that `Outer` implements `bytemuck::TransparentWrapper<Inner>`
pub struct TransparentWrapperProof<Outer: ?Sized, Inner: ?Sized>{
    _priv: PhantomData<(
        // Makes this invariant over the lifetimes in `Outer` and `Inner`,
        // so that the type parameters can't be coerced to a different lifetime.
        fn(PhantomData<Outer>) -> PhantomData<Outer>,
        fn(PhantomData<Inner>) -> PhantomData<Inner>,
    )>
}

impl<Outer: ?Sized, Inner: ?Sized> TransparentWrapperProof<Outer, Inner> {
    const __NEW: Self = Self{_priv: PhantomData};
    #[inline(always)]
    pub const unsafe fn new_unchecked() -> Self {
        Self::__NEW
    }
}

impl<A: ?Sized, B: ?Sized> Copy for TransparentWrapperProof<A, B> {}

impl<A: ?Sized, B: ?Sized> Clone for TransparentWrapperProof<A, B> {
    fn clone(&self) -> Self {
        *self
    }
}


#[doc(hidden)]
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! __check_size {
    ($transparent_wrapper_proof:expr, $panic:ident) => ({
        let proof = $transparent_wrapper_proof;
        if $crate::TransparentWrapperProof::is_not_same_size(proof) {
            proof.$panic()
        }
    })
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! __check_size {
    ($transparent_wrapper_proof:expr) => ()
}

#[cfg(debug_assertions)]
#[doc(hidden)]
impl<Outer: ?Sized, Inner: ?Sized> TransparentWrapperProof<Outer, Inner> {
    const NOT_SAME_SIZE: bool =
        core::mem::size_of::<*const Outer>() != core::mem::size_of::<*const Inner>();

    #[inline(always)]
    pub const fn is_not_same_size(self) -> bool {
        Self::NOT_SAME_SIZE
    }

    #[cold]
    #[inline(never)]
    pub const fn panic_peel(self) -> ! {
        transmute_unequal_ptr_size_panic(
            core::mem::size_of::<*const Outer>(),
            core::mem::size_of::<*const Inner>(),
        )
    }

    #[cold]
    #[inline(never)]
    pub const fn panic_wrap(self) -> ! {
        transmute_unequal_ptr_size_panic(
            core::mem::size_of::<*const Inner>(),
            core::mem::size_of::<*const Outer>(),
        )
    }
}


#[doc(hidden)]
#[cfg_attr(feature = "rust_1_57",track_caller)]
#[allow(unused_variables)]
#[cold]
#[inline(never)]
pub const fn transmute_unequal_ptr_size_panic(size_of_from: usize, size_of_to: usize) -> ! {
    crate::panic_!{
        {
            [/* expected transmute not to change the pointer size */][size_of_from]
        }
        {
            crate::const_panic::concat_panic!{
                "\nexpected transmute not to change the pointer size,",
                " size goes from: ", size_of_from,
                " to: ", size_of_to,
            }
        }
    }
}
