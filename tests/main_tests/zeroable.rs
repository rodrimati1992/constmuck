use constmuck::zeroed;

#[test]
fn zeroable_test() {
    macro_rules! case_inner {
        ($ty:ty, $zeroed:expr, [$($size:expr),*]) => ({
            {
                assert_eq!(zeroed::<$ty>(), $zeroed);
            }
            $({
                let arr: [$ty; $size] = zeroed();
                assert_eq!(arr, [$zeroed; $size]);
            })*
        })

    }
    macro_rules! case {
        ($ty:ty, $zeroed:expr) => {{
            case_inner! {
                $ty,
                $zeroed,
                [3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377]
            }
        }};
    }

    case! {i8, 0}
    case! {char, '\0'}
    case! {u32, 0}
    case! {*const u8, 0 as *const u8}
    case! {Option<&u32>, None}
}

#[test]
#[cfg_attr(not(feature = "rust_1_75"), should_panic)]
fn zeroed_of_large_type() {
    // spawning a thread to ensure that the stack has enough space for the array
    std::thread::Builder::new()
        .stack_size(5 * 1024 * 1024)
        .spawn(|| {
            let _ = zeroed::<[u8; 1_048_577]>();
        })
        .unwrap()
        .join()
        .unwrap();
}
