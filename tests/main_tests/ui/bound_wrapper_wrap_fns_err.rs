use bytemuck::TransparentWrapper;

struct Foo<T: ?Sized>(T);






fn _test_wrap<T>(a: T, b: T) -> (Foo<T>, Foo<T>) {
    (
        constmuck::wrapper::wrap::<Foo<T>, T>(a),
        <Foo<T> as TransparentWrapper<T>>::wrap(b),
    )
}




fn _test_wrap_ref<'a, T>(a: &'a T, b: &'a T, c: &'a T) -> (&'a Foo<T>, &'a Foo<T>, &'a Foo<T>) {
    (
        constmuck::wrapper::wrap_ref::<Foo<T>, T>(a),
        constmuck::wrapper::wrap_ref!(b, Foo<T>, T),
        <Foo<T> as TransparentWrapper<T>>::wrap_ref(c),
    )
}



fn _test_wrap_slice<'a, T>(a: &'a [T], b: &'a [T]) -> (&'a [Foo<T>], &'a [Foo<T>]) {
    (
        constmuck::wrapper::wrap_slice::<Foo<T>, T>(a),
        <Foo<T> as TransparentWrapper<T>>::wrap_slice(b),
    )
}



fn main(){}