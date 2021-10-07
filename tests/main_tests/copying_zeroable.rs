use super::test_utils::must_panic;

use constmuck::{copying, infer, type_size, zeroed, zeroed_array, TypeSize};

#[test]
fn test_copy() {
    #[cfg(feature = "debug_checks")]
    {
        must_panic(|| unsafe {
            let _ = copying::copy(
                &100u64,
                TypeSize::<_, u64, 1>::new_unchecked().with_bound(infer!()),
            );
        })
        .unwrap();
    }

    // reenable if it's sound to copy references like in 0.1.1
    // assert_eq!(copying::copy(&"hello", type_size!(&str)), "hello");

    assert_eq!(copying::copy(&10, type_size!(u32)), 10);

    // reenable if it's sound to copy references like in 0.1.1
    // let local = 13;
    // let reff = &local;
    // assert_eq!(copying::copy(&reff, type_size!(&u32)), &13);
}

#[test]
fn test_repeat() {
    #[cfg(feature = "debug_checks")]
    {
        must_panic(|| unsafe {
            let _: [_; 2] = copying::repeat(
                &"hello",
                TypeSize::<_, u64, 1>::new_unchecked().with_bound(infer!()),
            );
        })
        .unwrap();
    }

    macro_rules! case {
        ($size:expr) => {
            // reenable if it's sound to copy references like in 0.1.1
            // {
            //     let x: [_; $size] = copying::repeat(&"hello", type_size!(&str));
            //     assert_eq!(x, ["hello"; $size]);
            // }
            {
                let x: [_; $size] = copying::repeat(&10, type_size!(u32));
                assert_eq!(x, [10; $size]);
            }
            // reenable if it's sound to copy references like in 0.1.1
            // {
            //     let local = 13;
            //     let reff = &local;
            //     let x: [_; $size] = copying::repeat(&reff, type_size!(&u32));
            //     assert_eq!(x, [&13; $size]);
            // }
        };
    }
    case!(0);
    case!(1);
    case!(2);
    case!(3);
}

#[test]
fn zeroable_test() {
    macro_rules! case {
        ($ty:ty, $zeroed:expr, [$($size:expr),*]) => ({
            {
                assert_eq!(zeroed(type_size!($ty)), $zeroed);
            }
            $({
                let arr: [$ty; $size] = zeroed_array(type_size!($ty));
                assert_eq!(arr, [$zeroed; $size]);
            })*
        })
    }

    case! {u32, 0u32, [0, 1, 2, 3]}
    case! {*const u32, 0 as *const u32, [0, 1, 2, 3]}

    #[cfg(feature = "debug_checks")]
    {
        must_panic(|| unsafe {
            let _ = zeroed(TypeSize::<_, u64, 1>::new_unchecked().with_bound(infer!()));
        })
        .unwrap();

        must_panic(|| unsafe {
            let _: [u64; 2] =
                zeroed_array(TypeSize::<_, u64, 1>::new_unchecked().with_bound(infer!()));
        })
        .unwrap();
    }
}
