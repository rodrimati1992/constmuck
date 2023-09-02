fn _test_bytes_of<'a, T>(a: &'a T, b: &'a T) -> (&'a [u8], &'a [u8]) {
    (
        constmuck::bytes_of(a),
        bytemuck::bytes_of(b),
    )
}



fn _test_cast_slice_alt<'a, T, U>(a: &'a [T], b: &'a [T]) -> (&'a [U], &'a [U]) {
    (
        constmuck::cast_slice_alt(a),
        bytemuck::cast_slice(b),
    )
}




fn _test_try_cast_slice_alt<'a, T, U>(a: &'a [T], b: &'a [T]) -> (&'a [U], &'a [U]) {
    (
        constmuck::try_cast_slice_alt(a).unwrap(),
        bytemuck::try_cast_slice(b).unwrap(),
    )
}

fn main(){}
