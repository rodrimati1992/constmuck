use bytemuck::TransparentWrapper;

struct Foo<T: ?Sized>(T);






fn _test_peel<T>(a: Foo<T>, b: Foo<T>) -> (T, T) {
    (
        constmuck::wrapper::peel::<Foo<T>, T>(a),
        <Foo<T> as TransparentWrapper<T>>::peel(b),
    )
}




fn _test_peel_ref<'a, T>(a: &'a Foo<T>, b: &'a Foo<T>, c: &'a Foo<T>) -> (&'a T, &'a T, &'a T) {
    (
        constmuck::wrapper::peel_ref::<Foo<T>, T>(a),
        constmuck::wrapper::peel_ref!(b, Foo<T>, T),
        <Foo<T> as TransparentWrapper<T>>::peel_ref(c),
    )
}



fn _test_peel_slice<'a, T>(a: &'a [Foo<T>], b: &'a [Foo<T>]) -> (&'a [T], &'a [T]) {
    (
        constmuck::wrapper::peel_slice::<Foo<T>, T>(a),
        <Foo<T> as TransparentWrapper<T>>::peel_slice(b),
    )
}




fn main(){}