use super::test_utils::{must_panic, Pack, Wrap};

use constmuck::{
    infer,
    wrapper::{peel, peel_ref, peel_slice, wrap, wrap_ref, wrap_slice},
    Infer, IsTW, IsTransparentWrapper as ITW,
};

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
