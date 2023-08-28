use bytemuck::Contiguous;

fn _test_from_integer<T, U>(a: T, b: T) -> (U, U) {
    (
        constmuck::contiguous::from_integer(a).unwrap(),
        Contiguous::from_integer(b).unwrap(),
    )
}

fn _test_into_integer<T, U>(a: T, b: T) -> (U, U) {
    (
        constmuck::contiguous::into_integer(a),
        Contiguous::into_integer(b),
    )
}


fn main(){}
