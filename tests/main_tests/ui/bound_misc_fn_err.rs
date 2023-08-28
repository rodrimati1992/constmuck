fn _test_pod_read_unaligned<T>(a: &[u8], b: &[u8]) -> (T, T) {
    (
        constmuck::pod_read_unaligned(a),
        bytemuck::pod_read_unaligned(b),
    )
}



fn _test_try_pod_read_unaligned<T>(a: &[u8], b: &[u8]) -> (T, T) {
    (
        constmuck::try_pod_read_unaligned(a).unwrap(),
        bytemuck::try_pod_read_unaligned(b).unwrap(),
    )
}




fn _test_zeroed<T>() -> (T, T) {
    (
        constmuck::zeroed(),
        bytemuck::Zeroable::zeroed(),
    )
}


fn main(){}