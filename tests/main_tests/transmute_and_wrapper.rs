use super::test_utils::{must_panic, Pack, Wrap};

use constmuck::{
    infer,
    transmutable::{transmute_into, transmute_ref, transmute_slice},
    wrapper::{peel, peel_ref, peel_slice, wrap, wrap_ref, wrap_slice},
    Infer, IsTW, IsTransparentWrapper as ITW, TransmutableInto as TI,
};

#[test]
fn transmuteinto_pod_test() {
    // broadening alignment
    must_panic(|| TI::<Pack<u32>, u32>::pod(infer!())).unwrap();

    // equal alignment
    drop(TI::<u32, i32>::pod(infer!()));

    // narrowing alignment
    drop(TI::<u32, Pack<u32>>::pod(infer!()));

    // different size
    must_panic(|| TI::<u32, u64>::pod(infer!())).unwrap();
}

#[test]
fn transmuteinto_new_unchecked_test() {
    use self::transmute_into as fun;

    unsafe {
        assert_eq!(fun(127u8, TI::<u8, i8>::new_unchecked()), 127);
        assert_eq!(fun(128u8, TI::<u8, i8>::new_unchecked()), -128);
    }
}

#[test]
fn transmuteinto_join_test() {
    {
        let ti = TI::<u16, i16>::pod(infer!()).join(TI::<i16, [u8; 2]>::pod(infer!()));

        assert_eq!(transmute_into(127u16, ti), 127u16.to_ne_bytes());
        assert_eq!(transmute_into(128u16, ti), 128u16.to_ne_bytes());
        assert_eq!(transmute_into(2001u16, ti), 2001u16.to_ne_bytes());
        assert_eq!(transmute_into(2511u16, ti), 2511u16.to_ne_bytes());

        assert_eq!(transmute_ref(&127u16, ti), &127u16.to_ne_bytes());
        assert_eq!(transmute_ref(&128u16, ti), &128u16.to_ne_bytes());
        assert_eq!(transmute_ref(&2001u16, ti), &2001u16.to_ne_bytes());
        assert_eq!(transmute_ref(&2511u16, ti), &2511u16.to_ne_bytes());

        let input = [127u16, 128, 2001, 2511];
        assert_eq!(transmute_slice(&input, ti), &input.map(|x| x.to_ne_bytes()));
    }
    {
        #[derive(Debug, PartialEq)]
        #[repr(transparent)]
        struct Foo<T: ?Sized>(T);

        let ti = unsafe {
            TI::<str, Foo<str>>::new_unchecked().join(TI::<Foo<str>, Foo<[u8]>>::new_unchecked())
        };

        assert_eq!(transmute_ref!("hi", ti), &Foo(*b"hi") as &Foo<[u8]>);
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

    assert_eq!(fun(0u16, TI::<u16, Pack<u16>>::pod(infer!())), Pack(0));
    assert_eq!(
        fun(12345u16, TI::<u16, Pack<u16>>::pod(infer!())),
        Pack(12345)
    );
    assert_eq!(
        fun(u16::MAX, TI::<u16, Pack<u16>>::pod(infer!())),
        Pack(u16::MAX)
    );

    assert_eq!(fun(usize::MAX, TI::<usize, isize>::pod(infer!())), -1);
    assert_eq!(fun(0, TI::<usize, isize>::pod(infer!())), 0);
    assert_eq!(fun(2, TI::<usize, isize>::pod(infer!())), 2);

    assert_eq!(fun(9, IsTW!(Wrap<_>).from_inner), Wrap(9));
    assert_eq!(fun(Wrap(9), IsTW!(Wrap<_>).into_inner), 9);

    assert_eq!(
        fun([u8::MAX, 0, 1, 2], TI::<[u8; 4], [i8; 4]>::pod(infer!())),
        [-1i8, 0, 1, 2]
    );
    assert_eq!(
        fun([u8::MAX, 0, 1, 2], TI::<u8, i8>::pod(infer!()).array()),
        [-1i8, 0, 1, 2]
    );

    assert_eq!(
        fun([u8::MAX, 0, 1, 2], IsTW!(Wrap<_>).from_inner.array()),
        [u8::MAX, 0, 1, 2].map(Wrap),
    );
    assert_eq!(
        fun(
            [u8::MAX, 0, 1, 2].map(Wrap),
            IsTW!(Wrap<_>).into_inner.array()
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

        assert_eq!(fun $($b)? (&u16::MAX, TI::<u16, Pack<u16>>::pod(infer!())), &Pack(u16::MAX));
        assert_eq!(fun $($b)? (&0, TI::<u16, Pack<u16>>::pod(infer!())), &Pack(0));
        assert_eq!(fun $($b)? (&2, TI::<u16, Pack<u16>>::pod(infer!())), &Pack(2));

        assert_eq!(fun $($b)? (&usize::MAX, TI::<usize, isize>::pod(infer!())), &-1);
        assert_eq!(fun $($b)? (&0, TI::<usize, isize>::pod(infer!())), &0);
        assert_eq!(fun $($b)? (&2, TI::<usize, isize>::pod(infer!())), &2);

        assert_eq!(fun $($b)? (&9, IsTW!(Wrap<_>).from_inner), &Wrap(9));
        assert_eq!(fun $($b)? (&Wrap(9), IsTW!(Wrap<_>).into_inner), &9);

        assert_eq!(
            fun $($b)? (&[u8::MAX, 0, 1, 2], TI::<[u8; 4], [i8; 4]>::pod(infer!())),
            &[-1i8, 0, 1, 2]
        );
        assert_eq!(
            fun $($b)? (&[u8::MAX, 0, 1, 2], TI::<u8, i8>::pod(infer!()).array()),
            &[-1i8, 0, 1, 2]
        );

        assert_eq!(
            fun $($b)? (&[u8::MAX, 0, 1, 2], IsTW!(Wrap<_>).from_inner.array()),
            &[u8::MAX, 0, 1, 2].map(Wrap),
        );
        assert_eq!(
            fun $($b)? (
                &[u8::MAX, 0, 1, 2].map(Wrap),
                IsTW!(Wrap<_>).into_inner.array()
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
        fun::<_, Pack<_>>(
            &[u16::MAX, 0, 1, 2, 12345],
            TI::<u16, Pack<u16>>::pod(infer!())
        ),
        &[Pack(u16::MAX), Pack(0), Pack(1), Pack(2), Pack(12345)]
    );

    assert_eq!(
        fun::<_, Wrap<_>>(&[u8::MAX, 0, 1, 2], IsTW!().from_inner),
        &[Wrap(u8::MAX), Wrap(0), Wrap(1), Wrap(2)]
    );
    assert_eq!(
        fun(
            &[Wrap(u8::MAX), Wrap(0), Wrap(1), Wrap(2)],
            IsTW!().into_inner
        ),
        &[u8::MAX, 0, 1, 2]
    );

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(fun(&[128u8], TI::<u8, u64>::new_unchecked()))).unwrap();
    }
}

#[test]
fn wrapper_from_ti_test() {
    const ITW: ITW<u8, i8> = ITW::from_ti(TI::pod(infer!()), TI::pod(infer!()));

    assert_eq!(wrap(-1, ITW), 255);
    assert_eq!(wrap_ref(&-2, ITW), &254);
    assert_eq!(wrap_slice(&[-3, 3], ITW), &[253, 3]);
    assert_eq!(peel(128, ITW), -128);
    assert_eq!(peel_ref(&129, ITW), &-127);
    assert_eq!(peel_slice(&[254, 1], ITW), &[-2, 1]);
}

#[test]
fn wrapper_join_test() {
    use std::num::Wrapping as W;

    const ITW: ITW<W<W<u32>>, u32> = IsTW!(W<W<u32>>, W<u32>).join(IsTW!(W<u32>, u32));

    assert_eq!(wrap(3, ITW), W(W(3)));
    assert_eq!(wrap_ref(&5, ITW), &W(W(5)));
    assert_eq!(wrap_slice(&[8, 13], ITW), &[W(W(8)), W(W(13))][..]);
    assert_eq!(peel(W(W(21)), ITW), 21);
    assert_eq!(peel_ref(&W(W(34)), ITW), &34);
    assert_eq!(peel_slice(&[W(W(55)), W(W(89))], ITW), &[55, 89][..]);
}

#[test]
fn peel_test() {
    assert_eq!(peel(Wrap("hello"), infer!()), "hello");
    assert_eq!(peel(Wrap("hello"), Infer::INFER), "hello");
    assert_eq!(peel(Wrap("foo"), IsTW!()), "foo");
    assert_eq!(peel(Wrap(false), IsTW!(Wrap<_>)), false);
    assert_eq!(peel(Wrap('A'), IsTW!(Wrap<_>,)), 'A');
    assert_eq!(peel(Wrap(b"baz"), IsTW!(Wrap<_>, _)), b"baz");
    assert_eq!(peel(Wrap("baz"), IsTW!(Wrap<_>, _,)), "baz");

    assert_eq!(peel(Wrap(["hello", "world"]), IsTW!()), ["hello", "world"]);
    assert_eq!(
        peel(["hello", "world"].map(Wrap), IsTW!().array()),
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
        assert_eq!(peel_ref $($b)? (&Wrap(true), IsTW!()), &true);
        assert_eq!(peel_ref $($b)? (&Wrap(100), IsTW!()), &100);

        assert_eq!(peel_ref $($b)? (&Wrap([100, 200]), IsTW!()), &[100, 200]);
        assert_eq!(
            peel_ref $($b)? (&[100, 200].map(Wrap), IsTW!().array()),
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
    assert_eq!(wrap("hello", IsTW!(Wrap<_>)), Wrap("hello"));
    assert_eq!(
        wrap(["hello", "world"], IsTW!(Wrap<_>)),
        Wrap(["hello", "world"])
    );
    assert_eq!(
        wrap(["hello", "world"], IsTW!(Wrap<_>).array()),
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
        assert_eq!(wrap_ref $($b)? (&true, IsTW!(Wrap<_>)), &Wrap(true));
        assert_eq!(wrap_ref $($b)? (&100, IsTW!(Wrap<_>)), &Wrap(100));

        assert_eq!(wrap_ref $($b)? (&[100, 200], IsTW!(Wrap<_>)), &Wrap([100, 200]));
        assert_eq!(
            wrap_ref $($b)? (&[100, 200], IsTW!(Wrap<_>).array()),
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
        wrap_slice(&[true, false], IsTW!(Wrap<_>)),
        &[true, false].map(Wrap)
    );
    assert_eq!(
        wrap_slice(&[123, 456], IsTW!(Wrap<_>)),
        &[123, 456].map(Wrap)
    );

    #[cfg(feature = "debug_checks")]
    unsafe {
        must_panic(|| drop(wrap_slice(&[10], ITW::<Wrap<u8>, u16>::new_unchecked()))).unwrap();
    }
}
