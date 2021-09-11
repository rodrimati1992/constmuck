use constmuck::{copying, infer, type_size, zeroed, zeroed_array};

#[test]
fn test_copy() {
    assert_eq!(copying::copy(&"hello", type_size!(&str)), "hello");
    assert_eq!(copying::copy(&10, type_size!(u32)), 10);

    let local = 13;
    let reff = &local;
    assert_eq!(copying::copy(&reff, type_size!(&u32)), &13);
}

#[test]
fn test_repeat() {
    macro_rules! case {
        ($size:expr) => {{
            let x: [_; $size] = copying::repeat(&"hello", type_size!(&str));
            assert_eq!(x, ["hello"; $size]);
        }
        {
            let x: [_; $size] = copying::repeat(&10, type_size!(u32));
            assert_eq!(x, [10; $size]);
        }
        {
            let local = 13;
            let reff = &local;
            let x: [_; $size] = copying::repeat(&reff, type_size!(&u32));
            assert_eq!(x, [&13; $size]);
        }};
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
}
