use constmuck::zeroed;

#[test]
fn zeroable_test() {
    macro_rules! case {
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

    case! {u32, 0u32, [0, 1, 2, 3]}
    case! {*const u32, 0 as *const u32, [0, 1, 2, 3]}
}
