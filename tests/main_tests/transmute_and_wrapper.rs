use super::test_utils::{must_panic, Pack, Wrap};

use constmuck::{
    infer, infer_tw,
    transmutable::{transmute_into, transmute_ref, transmute_slice},
    wrapper::{peel, peel_ref, peel_slice, wrap, wrap_ref, wrap_slice},
    ImplsTransparentWrapper as ITW, Infer, TransmutableInto as TI,
};

#[test]
fn transmute_into_pod_test() {
    // different alignment
    must_panic(|| TI::<Pack<u32>, u32>::pod(infer!())).unwrap();

    // different size
    must_panic(|| TI::<u32, u64>::pod(infer!())).unwrap();
}

#[test]
fn transmute_into_new_unchecked_test() {
    use self::transmute_into as fun;

    unsafe {
        assert_eq!(fun(127u8, TI::<u8, i8>::new_unchecked()), 127);
        assert_eq!(fun(128u8, TI::<u8, i8>::new_unchecked()), -128);
    }
}

#[test]
fn transparent_wrapper_new_unchecked_test() {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(transparent)]
    pub struct W<T>(pub T);

    unsafe {
        assert_eq!(wrap(127u8, ITW::<W<u8>, u8>::new_unchecked()), W(127u8));
        assert_eq!(wrap(128u8, ITW::<W<u8>, u8>::new_unchecked()), W(128u8));

        assert_eq!(peel(W(21u8), ITW::<W<u8>, u8>::new_unchecked()), 21u8);
        assert_eq!(peel(W(37u8), ITW::<W<u8>, u8>::new_unchecked()), 37u8);
    }
}

#[test]
fn transmute_into_test() {
    use self::transmute_into as fun;

    assert_eq!(fun(usize::MAX, TI::<usize, isize>::pod(infer!())), -1);
    assert_eq!(fun(0, TI::<usize, isize>::pod(infer!())), 0);
    assert_eq!(fun(2, TI::<usize, isize>::pod(infer!())), 2);

    assert_eq!(fun(9, infer_tw!(Wrap<_>).from_inner), Wrap(9));
    assert_eq!(fun(Wrap(9), infer_tw!(Wrap<_>).into_inner), 9);

    assert_eq!(
        fun([u8::MAX, 0, 1, 2], TI::<[u8; 4], [i8; 4]>::pod(infer!())),
        [-1i8, 0, 1, 2]
    );
    assert_eq!(
        fun([u8::MAX, 0, 1, 2], TI::<u8, i8>::pod(infer!()).array()),
        [-1i8, 0, 1, 2]
    );

    assert_eq!(
        fun([u8::MAX, 0, 1, 2], infer_tw!(Wrap<_>).from_inner.array()),
        [u8::MAX, 0, 1, 2].map(Wrap),
    );
    assert_eq!(
        fun(
            [u8::MAX, 0, 1, 2].map(Wrap),
            infer_tw!(Wrap<_>).into_inner.array()
        ),
        [u8::MAX, 0, 1, 2],
    );

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(fun(128u8, TI::<u8, u64>::new_unchecked()))).unwrap();
    }
}

#[test]
fn transmute_ref_test() {
    use self::transmute_ref as fun;

    macro_rules! test_fn_or_macro {($($b:tt)?) => (

        assert_eq!(fun $($b)? (&usize::MAX, TI::<usize, isize>::pod(infer!())), &-1);
        assert_eq!(fun $($b)? (&0, TI::<usize, isize>::pod(infer!())), &0);
        assert_eq!(fun $($b)? (&2, TI::<usize, isize>::pod(infer!())), &2);

        assert_eq!(fun $($b)? (&9, infer_tw!(Wrap<_>).from_inner), &Wrap(9));
        assert_eq!(fun $($b)? (&Wrap(9), infer_tw!(Wrap<_>).into_inner), &9);

        assert_eq!(
            fun $($b)? (&[u8::MAX, 0, 1, 2], TI::<[u8; 4], [i8; 4]>::pod(infer!())),
            &[-1i8, 0, 1, 2]
        );
        assert_eq!(
            fun $($b)? (&[u8::MAX, 0, 1, 2], TI::<u8, i8>::pod(infer!()).array()),
            &[-1i8, 0, 1, 2]
        );

        assert_eq!(
            fun $($b)? (&[u8::MAX, 0, 1, 2], infer_tw!(Wrap<_>).from_inner.array()),
            &[u8::MAX, 0, 1, 2].map(Wrap),
        );
        assert_eq!(
            fun $($b)? (
                &[u8::MAX, 0, 1, 2].map(Wrap),
                infer_tw!(Wrap<_>).into_inner.array()
            ),
            &[u8::MAX, 0, 1, 2],
        );
    )}

    test_fn_or_macro! {/*fn*/}
    test_fn_or_macro! {/*macro*/ !}

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(fun(&128u8, TI::<u8, u64>::new_unchecked()))).unwrap();
        must_panic(|| {
            // different pointer sizes
            drop(transmute_ref!("hello", TI::<str, ()>::new_unchecked()))
        })
        .unwrap();
    }
}

#[test]
fn transmute_slice_test() {
    use self::transmute_slice as fun;

    assert_eq!(
        fun(&[u8::MAX, 0, 1, 2], TI::<u8, i8>::pod(infer!())),
        &[-1i8, 0, 1, 2]
    );

    assert_eq!(
        fun::<_, Wrap<_>>(&[u8::MAX, 0, 1, 2], infer_tw!().from_inner),
        &[Wrap(u8::MAX), Wrap(0), Wrap(1), Wrap(2)]
    );
    assert_eq!(
        fun(
            &[Wrap(u8::MAX), Wrap(0), Wrap(1), Wrap(2)],
            infer_tw!().into_inner
        ),
        &[u8::MAX, 0, 1, 2]
    );

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(fun(&[128u8], TI::<u8, u64>::new_unchecked()))).unwrap();
    }
}

#[test]
fn peel_test() {
    assert_eq!(peel(Wrap("hello"), infer!()), "hello");
    assert_eq!(peel(Wrap("hello"), Infer::INFER), "hello");
    assert_eq!(peel(Wrap("foo"), infer_tw!()), "foo");
    assert_eq!(peel(Wrap(false), infer_tw!(Wrap<_>)), false);
    assert_eq!(peel(Wrap('A'), infer_tw!(Wrap<_>,)), 'A');
    assert_eq!(peel(Wrap(b"baz"), infer_tw!(Wrap<_>, _)), b"baz");
    assert_eq!(peel(Wrap("baz"), infer_tw!(Wrap<_>, _,)), "baz");

    assert_eq!(
        peel(Wrap(["hello", "world"]), infer_tw!()),
        ["hello", "world"]
    );
    assert_eq!(
        peel(["hello", "world"].map(Wrap), infer_tw!().array()),
        ["hello", "world"]
    );

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(peel(Wrap("baz"), ITW::<_, u8>::new_unchecked()))).unwrap();
    }
}

#[test]
fn peel_ref_test() {
    macro_rules! test_fn_or_macro {($($b:tt)?) => (
        assert_eq!(peel_ref $($b)? (&Wrap(true), infer_tw!()), &true);
        assert_eq!(peel_ref $($b)? (&Wrap(100), infer_tw!()), &100);

        assert_eq!(peel_ref $($b)? (&Wrap([100, 200]), infer_tw!()), &[100, 200]);
        assert_eq!(
            peel_ref $($b)? (&[100, 200].map(Wrap), infer_tw!().array()),
            &[100, 200]
        );
    )}

    test_fn_or_macro! {/*fn*/}
    test_fn_or_macro! {/*macro*/ !}

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(peel_ref(&Wrap("baz"), ITW::<_, u8>::new_unchecked()))).unwrap();
        must_panic(|| {
            // different pointer sizes
            let x: &Wrap<[u8]> = &Wrap([0]);
            drop(peel_ref!(x, ITW::<Wrap<[u8]>, ()>::new_unchecked()))
        })
        .unwrap();
    }
}

#[test]
fn peel_slice_test() {
    assert_eq!(
        peel_slice(&[true, false].map(Wrap), infer!()),
        &[true, false]
    );
    assert_eq!(peel_slice(&[123, 456].map(Wrap), infer!()), &[123, 456]);

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(peel_slice(&[Wrap("baz")], ITW::<_, u8>::new_unchecked()))).unwrap();
    }
}

#[test]
fn wrap_test() {
    assert_eq!(wrap("hello", infer_tw!(Wrap<_>)), Wrap("hello"));
    assert_eq!(
        wrap(["hello", "world"], infer_tw!(Wrap<_>)),
        Wrap(["hello", "world"])
    );
    assert_eq!(
        wrap(["hello", "world"], infer_tw!(Wrap<_>).array()),
        ["hello", "world"].map(Wrap)
    );

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(wrap(10, ITW::<Wrap<u8>, u16>::new_unchecked()))).unwrap();
    }
}

#[test]
fn wrap_ref_test() {
    macro_rules! test_fn_or_macro {($($b:tt)?) => (
        assert_eq!(wrap_ref $($b)? (&true, infer_tw!(Wrap<_>)), &Wrap(true));
        assert_eq!(wrap_ref $($b)? (&100, infer_tw!(Wrap<_>)), &Wrap(100));

        assert_eq!(wrap_ref $($b)? (&[100, 200], infer_tw!(Wrap<_>)), &Wrap([100, 200]));
        assert_eq!(
            wrap_ref $($b)? (&[100, 200], infer_tw!(Wrap<_>).array()),
            &[100, 200].map(Wrap)
        );
    )}

    test_fn_or_macro! {/*fn*/}
    test_fn_or_macro! {/*macro*/ !}

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(wrap_ref(&10, ITW::<Wrap<u8>, u16>::new_unchecked()))).unwrap();

        must_panic(|| {
            // different pointer sizes
            let x: &[u8] = &[0];
            drop(wrap_ref!(x, ITW::<Wrap<u8>, [u8]>::new_unchecked()))
        })
        .unwrap();
    }
}

#[test]
fn wrap_slice_test() {
    assert_eq!(
        wrap_slice(&[true, false], infer_tw!(Wrap<_>)),
        &[true, false].map(Wrap)
    );
    assert_eq!(
        wrap_slice(&[123, 456], infer_tw!(Wrap<_>)),
        &[123, 456].map(Wrap)
    );

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(wrap_slice(&[10], ITW::<Wrap<u8>, u16>::new_unchecked()))).unwrap();
    }
}
