fn _test_cast<T, U>(a: T, b: T) -> (U, U) {
    (
        constmuck::cast(a),
        bytemuck::cast(b),
    )
}



fn _test_try_cast<T, U>(a: T, b: T) -> (U, U) {
    (
        constmuck::try_cast(a).unwrap(),
        bytemuck::try_cast(b).unwrap(),
    )
}




fn _test_cast_ref_alt<'a, T, U>(a: &'a T, b: &'a T) -> (&'a U, &'a U) {
    (
        constmuck::cast_ref_alt(a),
        bytemuck::cast_ref(b),
    )
}




fn _test_try_cast_ref_alt<'a, T, U>(a: &'a T, b: &'a T) -> (&'a U, &'a U) {
    (
        constmuck::try_cast_ref_alt(a).unwrap(),
        bytemuck::try_cast_ref(b).unwrap(),
    )
}

fn main(){}