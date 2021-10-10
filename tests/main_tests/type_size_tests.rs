use super::test_utils::must_panic;

use constmuck::{
    cast, copying, infer, map_bound, type_size, zeroed, IsCopy, IsPod, IsZeroable, TypeSize,
};

#[test]
fn map_bound_test() {
    let bound: TypeSize<IsPod<u32>, u32, 4> = type_size!(u32);

    assert_eq!(cast::<u32, i32>(100, (bound.into_bounds(), infer!())), 100);
    assert_eq!(cast::<u32, i32>(100, (*bound.bounds(), infer!())), 100);

    assert_eq!(
        copying::copy(&12345u32, map_bound!(bound, |x| x.impls_copy)),
        12345
    );

    assert_eq!(zeroed(map_bound!(bound, |x| x.impls_zeroable)), 0u32);
}

#[test]
fn new_unchecked_test() {
    #[derive(Debug, PartialEq)]
    #[repr(transparent)]
    struct WrapNI<T>(T);

    unsafe {
        let ts = TypeSize::<_, WrapNI<u32>, 4>::new_unchecked()
            .with_bound(IsZeroable::<WrapNI<u32>>::new_unchecked());

        assert_eq!(zeroed(ts), WrapNI(0u32));
    }
    unsafe {
        const STR_SIZE: usize = std::mem::size_of::<&str>();
        let ts = TypeSize::<_, WrapNI<&str>, STR_SIZE>::new_unchecked()
            .with_bound(IsCopy::<WrapNI<&str>>::new_unchecked());

        assert_eq!(copying::copy(&WrapNI("hello"), ts), WrapNI("hello"));
    }
}
#[test]
fn construction_test() {
    must_panic(|| {
        let _: TypeSize<(), u32, 5> = TypeSize::new_panicking();
    })
    .unwrap();

    let _: TypeSize<(), u32, 4> = TypeSize::new_panicking();

    let _: TypeSize<(), u32, 5> = unsafe { TypeSize::new_unchecked() };
    let _: TypeSize<(), u32, 4> = unsafe { TypeSize::new_unchecked() };
    let _: TypeSize<(), u32, 3> = unsafe { TypeSize::new_unchecked() };
}

#[test]
fn bound_manip() {
    let bound: TypeSize<IsPod<u32>, u32, 4> = type_size!(u32);

    assert_eq!(
        cast::<u32, i32>(12345u32, (bound.split().0, infer!())),
        12345
    );
    let _: TypeSize<(), u32, 4> = bound.split().1;

    assert_eq!(
        copying::copy(&12345u32, type_size!(u32).with_bound(infer!())),
        12345
    );
    let _: TypeSize<(), u32, 4> = type_size!(u32).with_bound(());
    let _: TypeSize<IsPod<u64>, u32, 4> = type_size!(u32).with_bound(IsPod::NEW);

    assert_eq!(zeroed(map_bound!(bound, |x| x.impls_zeroable)), 0u32);

    assert_eq!(zeroed(bound.set_bound(IsZeroable::NEW)), 0u32);
}
