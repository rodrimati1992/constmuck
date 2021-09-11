use std::num::Wrapping;

use static_assertions::{assert_impl_one, assert_not_impl_all};

use constmuck::{
    type_size, ImplsContiguous, ImplsCopy, ImplsPod, ImplsTransparentWrapper, ImplsZeroable, Infer,
    TypeSize,
};

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
fn impls_contiguous_construction() {
    assert_type_name(ImplsContiguous::<u32, u32>::NEW, "Impls");
    assert_type_name(ImplsContiguous::<NoTraits, NoTraits>::NEW, "Unit");

    assert_impl_one! {ImplsContiguous<u32, u32>: Infer}
    assert_not_impl_all! {ImplsContiguous<u32, u64>: Infer}
}

#[test]
fn impls_copy_construction() {
    assert_type_name(ImplsCopy::<u32>::NEW, "Impls");
    assert_type_name(ImplsCopy::<NoTraits>::NEW, "Unit");

    assert_impl_one! {ImplsCopy<u32>: Infer}
    assert_not_impl_all! {ImplsCopy<NoTraits>: Infer}
}

#[test]
fn impls_pod_construction() {
    assert_type_name(ImplsPod::<u32>::NEW, "Impls");
    assert_type_name(ImplsPod::<NoTraits>::NEW, "Unit");

    assert_impl_one! {ImplsPod<u32>: Infer}
    assert_not_impl_all! {ImplsPod<NoTraits>: Infer}
}

#[test]
fn impls_transparent_wrapper_construction() {
    assert_type_name(ImplsTransparentWrapper::<Wrapping<u32>, u32>::NEW, "Impls");
    assert_type_name(ImplsTransparentWrapper::<Wrapping<u32>, u64>::NEW, "Unit");
    assert_type_name(ImplsTransparentWrapper::<NoTraits, NoTraits>::NEW, "Unit");

    assert_impl_one!(ImplsTransparentWrapper<Wrapping<u32>, u32>: Infer);
    assert_not_impl_all!(ImplsTransparentWrapper<Wrapping<u32>, u64>: Infer);
    assert_not_impl_all!(ImplsTransparentWrapper<NoTraits, NoTraits>: Infer);
}

#[test]
fn impls_zeroable_construction() {
    assert_type_name(ImplsZeroable::<u32>::NEW, "Impls");
    assert_type_name(ImplsZeroable::<NoTraits>::NEW, "Unit");

    assert_impl_one! {ImplsZeroable<u32>: Infer}
    assert_not_impl_all! {ImplsZeroable<NoTraits>: Infer}
}

#[test]
fn type_size_construction() {
    // <_< not much of a black box test anymore, huh?

    let impls: TypeSize<(), u32, 4> = type_size!(u32);

    assert_type_name(impls, "TypeSize");
    assert_type_name(
        TypeSize::<(), u32, 4>::__13878307735224946849NEW__,
        "TypeSize",
    );
    assert_type_name(
        TypeSize::<NoTraits, u32, 4>::__13878307735224946849NEW__,
        "Unit",
    );

    // TypeSize never implements Infer
    assert_not_impl_all! {TypeSize<(), u32, 4>: Infer}
}
