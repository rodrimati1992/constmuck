// this ensures that the lifetime of the type parameters to `Is*` types are invariant.
// 
// Each function must correspond to exactly one lifetime error,
// currently that error is:
// ```text
// does not necessarily outlive the static lifetime
// ```
use constmuck::{IsContiguous, IsCopy, IsPod, IsTransparentWrapper, IsZeroable, TypeSize};

fn is_contiguous_as<'a, T>(x: IsContiguous<&'a T, u8>) -> IsContiguous<&'static T, u8> {
    x
}
fn is_contiguous_sa<'a, T>(x: IsContiguous<&'static T, u8>) -> IsContiguous<&'a T, u8> {
    x
}

fn is_copy_as<'a, T>(x: IsCopy<&'a T>) -> IsCopy<&'static T> {
    x
}
fn is_copy_sa<'a, T>(x: IsCopy<&'static T>) -> IsCopy<&'a T> {
    x
}

fn is_pod_as<'a, T>(x: IsPod<&'a T>) -> IsPod<&'static T> {
    x
}
fn is_pod_sa<'a, T>(x: IsPod<&'static T>) -> IsPod<&'a T> {
    x
}

fn is_transparent_wrapper_aasa<'a, T>(
    x: IsTransparentWrapper<&'a T, &'a T>,
) -> IsTransparentWrapper<&'static T, &'a T> {
    x
}
fn is_transparent_wrapper_saaa<'a, T>(
    x: IsTransparentWrapper<&'static T, &'a T>,
) -> IsTransparentWrapper<&'a T, &'a T> {
    x
}

fn is_transparent_wrapper_aaas<'a, T>(
    x: IsTransparentWrapper<&'a T, &'a T>,
) -> IsTransparentWrapper<&'a T, &'static T> {
    x
}
fn is_transparent_wrapper_asaa<'a, T>(
    x: IsTransparentWrapper<&'a T, &'static T>,
) -> IsTransparentWrapper<&'a T, &'a T> {
    x
}

fn is_zeroable_as<'a, T>(x: IsZeroable<&'a T>) -> IsZeroable<&'static T> {
    x
}
fn is_zeroable_sa<'a, T>(x: IsZeroable<&'static T>) -> IsZeroable<&'a T> {
    x
}

fn type_size_as<'a, T>(x: TypeSize<&'a T, (), 0>) -> TypeSize<&'static T, (), 0> {
    x
}
fn type_size_sa<'a, T>(x: TypeSize<&'static T, (), 0>) -> TypeSize<&'a T, (), 0> {
    x
}


fn main(){}