use std::num::Wrapping;

use static_assertions::{assert_impl_one, assert_not_impl_all};

use constmuck::{Infer, IsContiguous, IsCopy, IsPod, IsTransparentWrapper, IsZeroable, TypeSize};

struct NoTraits;

struct Unit;

trait New {
    const NEW: Unit = Unit;

    // For testing TypeSize
    const __13878307735224946849NEW__: Unit = Unit;
}

impl<T> New for T {}

#[track_caller]
fn assert_type_name<T>(_: T, name: &str) {
    let tn = std::any::type_name::<T>();
    assert!(
        tn.contains(name),
        "actual: {}\nexpected contained: {}",
        tn,
        name
    );
}

#[test]
fn is_contiguous_construction() {
    assert_type_name(IsContiguous::<u32, u32>::NEW, "Is");
    assert_type_name(IsContiguous::<NoTraits, NoTraits>::NEW, "Unit");

    assert_impl_one! {IsContiguous<u32, u32>: Infer}
    assert_not_impl_all! {IsContiguous<u32, u64>: Infer}
}

#[test]
fn is_copy_construction() {
    assert_type_name(IsCopy::<u32>::NEW, "Is");
    assert_type_name(IsCopy::<NoTraits>::NEW, "Unit");

    assert_impl_one! {IsCopy<u32>: Infer}
    assert_not_impl_all! {IsCopy<NoTraits>: Infer}
}

#[test]
fn is_pod_construction() {
    assert_type_name(IsPod::<u32>::NEW, "Is");
    assert_type_name(IsPod::<NoTraits>::NEW, "Unit");

    assert_impl_one! {IsPod<u32>: Infer}
    assert_not_impl_all! {IsPod<NoTraits>: Infer}
}

#[test]
fn is_transparent_wrapper_construction() {
    assert_type_name(IsTransparentWrapper::<Wrapping<u32>, u32>::NEW, "Is");
    assert_type_name(IsTransparentWrapper::<Wrapping<u32>, u64>::NEW, "Unit");
    assert_type_name(IsTransparentWrapper::<NoTraits, NoTraits>::NEW, "Unit");

    assert_impl_one!(IsTransparentWrapper<Wrapping<u32>, u32>: Infer);
    assert_not_impl_all!(IsTransparentWrapper<Wrapping<u32>, u64>: Infer);
    assert_not_impl_all!(IsTransparentWrapper<NoTraits, NoTraits>: Infer);
}

#[test]
fn is_zeroable_construction() {
    assert_type_name(IsZeroable::<u32>::NEW, "Is");
    assert_type_name(IsZeroable::<NoTraits>::NEW, "Unit");

    assert_impl_one! {IsZeroable<u32>: Infer}
    assert_not_impl_all! {IsZeroable<NoTraits>: Infer}
}

#[test]
fn type_size_construction() {
    // <_< not much of a black box test anymore, huh?

    let impls: TypeSize<u32, (), 4> = TypeSize!(u32);

    assert_type_name(impls, "TypeSize");
    assert_type_name(
        TypeSize::<u32, (), 4>::__13878307735224946849NEW__,
        "TypeSize",
    );
    assert_type_name(
        TypeSize::<u32, NoTraits, 4>::__13878307735224946849NEW__,
        "Unit",
    );

    // TypeSize never implements Infer
    assert_not_impl_all! {TypeSize<u32, (), 4>: Infer}
}
