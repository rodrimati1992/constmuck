use super::test_utils::{must_panic, Pack, Wrap};

use constmuck::wrapper::{
    peel, peel_ref, peel_slice, wrap, wrap_ref, wrap_slice, TransparentWrapper,
};

#[cfg(feature = "debug_checks")]
// for testing that `feature = "debug_checks"` rejects
// peeling from or wrapping into this type,
// which is 0-sized, but with the same alignment as `T`.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UnitW<T>([T; 0]);

#[cfg(feature = "debug_checks")]
unsafe impl<T> TransparentWrapper<T> for UnitW<T> {}

#[test]
fn peel_test() {
    #[cfg(all(miri, feature = "debug_checks"))]
    {
        must_panic(|| peel(Pack(0u16))).unwrap();
        must_panic(|| peel(UnitW::<u16>([]))).unwrap();
    }

    assert_eq!(peel(Wrap("hello")), "hello");
    assert_eq!(peel(Wrap("foo")), "foo");
    assert_eq!(peel(Wrap(false)), false);
    assert_eq!(peel(Wrap('A')), 'A');
    assert_eq!(peel(Wrap(b"baz")), b"baz");
    assert_eq!(peel(Wrap("baz")), "baz");
}

#[test]
fn peel_ref_test() {
    #[cfg(all(miri, feature = "debug_checks"))]
    {
        must_panic(|| peel_ref(&Pack(0u16))).unwrap();
        must_panic(|| peel_ref(&UnitW::<u16>([]))).unwrap();
    }

    assert_eq!(peel_ref(&Wrap(true)), &true);
    assert_eq!(peel_ref(&Wrap(100)), &100);
    assert_eq!(peel_ref(&Wrap([100, 200])), &[100, 200]);
}

#[test]
fn peel_ref_macro_test() {
    {
        assert_eq!(peel_ref!(&Wrap(true)), &true);
        assert_eq!(peel_ref!(&Wrap(100)), &100);
        assert_eq!(peel_ref!(&Wrap([100, 200])), &[100, 200]);
    }
    {
        assert_eq!(peel_ref!(&Wrap(true), Wrap<_>), &true);
        assert_eq!(peel_ref!(&Wrap(100), Wrap<_>), &100);
        assert_eq!(peel_ref!(&Wrap([100, 200]), Wrap<_>), &[100, 200]);
    }
    {
        assert_eq!(peel_ref!(&Wrap(true), Wrap<bool>, bool), &true);
        assert_eq!(peel_ref!(&Wrap(100), Wrap<u8>, u8), &100);
        assert_eq!(
            peel_ref!(&Wrap([100, 200]), Wrap<[u8; 2]>, [u8; 2]),
            &[100, 200]
        );
    }
}

#[test]
fn peel_slice_test() {
    #[cfg(all(miri, feature = "debug_checks"))]
    {
        must_panic(|| peel_slice(&[Pack(0u16)])).unwrap();
        must_panic(|| peel_slice(&[UnitW::<u16>([])])).unwrap();
    }

    assert_eq!(peel_slice(&[true, false].map(Wrap)), &[true, false]);
    assert_eq!(peel_slice(&[123, 456].map(Wrap)), &[123, 456]);
}

#[test]
fn wrap_test() {
    #[cfg(all(miri, feature = "debug_checks"))]
    {
        must_panic(|| wrap::<Pack<_>, _>(0u16)).unwrap();
        must_panic(|| wrap::<UnitW<_>, _>(0u16)).unwrap();
    }

    assert_eq!(wrap::<Wrap<_>, _>("hello"), Wrap("hello"));
    assert_eq!(
        wrap::<Wrap<_>, _>(["hello", "world"]),
        Wrap(["hello", "world"])
    );
}

#[test]
fn wrap_ref_test() {
    #[cfg(all(miri, feature = "debug_checks"))]
    {
        must_panic(|| wrap_ref::<Pack<_>, _>(&0u16)).unwrap();
        must_panic(|| wrap_ref::<UnitW<_>, _>(&0u16)).unwrap();
    }

    assert_eq!(wrap_ref::<Wrap<_>, _>(&true), &Wrap(true));
    assert_eq!(wrap_ref::<Wrap<_>, _>(&100), &Wrap(100));
    assert_eq!(wrap_ref::<Wrap<_>, _>(&[100, 200]), &Wrap([100, 200]));
}

#[test]
fn wrap_ref_macro_test() {
    {
        let v: &Wrap<_> = wrap_ref!(&true);
        assert_eq!(v, &Wrap(true));
    }
    {
        let v: &Wrap<_> = wrap_ref!(&100);
        assert_eq!(v, &Wrap(100));
    }
    {
        let v: &Wrap<_> = wrap_ref!(&[100, 200]);
        assert_eq!(v, &Wrap([100, 200]));
    }

    {
        assert_eq!(wrap_ref!(&true, Wrap<_>), &Wrap(true));
        assert_eq!(wrap_ref!(&100, Wrap<_>), &Wrap(100));
        assert_eq!(wrap_ref!(&[100, 200], Wrap<_>), &Wrap([100, 200]));
    }

    {
        assert_eq!(wrap_ref!(&true, Wrap<bool>, bool), &Wrap(true));
        assert_eq!(wrap_ref!(&100, Wrap<u32>, u32), &Wrap(100));
        assert_eq!(
            wrap_ref!(&[100, 200], Wrap<[u8; 2]>, [u8; 2]),
            &Wrap([100, 200])
        );
    }
}

#[test]
fn wrap_slice_test() {
    #[cfg(all(miri, feature = "debug_checks"))]
    {
        must_panic(|| wrap_slice::<Pack<_>, _>(&[0u16])).unwrap();
        must_panic(|| wrap_slice::<UnitW<_>, _>(&[0u16])).unwrap();
    }

    assert_eq!(
        wrap_slice::<Wrap<_>, _>(&[true, false]),
        &[true, false].map(Wrap)
    );
    assert_eq!(wrap_slice::<Wrap<_>, _>(&[123, 456]), &[123, 456].map(Wrap));
}
