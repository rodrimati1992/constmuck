use super::test_utils::{must_panic, Pack};

use constmuck::{
    byte_array_of, cast, cast_ref_alt, cast_slice_alt as csa, infer, try_cast, try_cast_ref_alt,
    try_cast_slice_alt as tcsa, Infer, IsPod,
    PodCastError::{SizeMismatch, TargetAlignmentGreaterAndInputNotAligned},
    TypeSize,
};

#[test]
fn bytes_of_test() {
    assert_eq!(
        byte_array_of(&[-2, -1, 0, 1, 2], TypeSize!([i8; 5])),
        &[254, 255, 0, 1, 2]
    );
    assert_eq!(byte_array_of(&[0, 1, 2], TypeSize!([u8; 3])), &[0, 1, 2]);
    assert_eq!(
        byte_array_of(&123456789u32, TypeSize!(u32)),
        &123456789u32.to_ne_bytes()
    );
}

#[test]
fn cast_test() {
    must_panic(|| cast(0, infer!((IsPod<u32>, IsPod<u16>)))).unwrap();

    assert_eq!(cast::<u32, i32>(u32::MAX, infer!()), -1i32);

    unsafe {
        assert_eq!(
            cast::<u8, i8>(250, (IsPod::new_unchecked(), IsPod::new_unchecked())),
            -6i8
        );
    }
}

#[test]
fn try_cast_test() {
    assert_eq!(
        try_cast(0, infer!((IsPod<u32>, IsPod<u16>))),
        Err(SizeMismatch)
    );
    assert_eq!(try_cast::<u32, Pack<u32>>(1, infer!()), Ok(Pack(1)));
    assert_eq!(try_cast::<u32, i32>(u32::MAX, infer!()), Ok(-1i32));
}

#[test]
fn cast_ref_alt_test() {
    must_panic(|| cast_ref_alt(&0, infer!((IsPod<u32>, IsPod<u16>)))).unwrap();
    must_panic(|| cast_ref_alt::<Pack<u32>, u32>(&Pack(0), infer!())).unwrap();
    assert_eq!(
        cast_ref_alt::<u32, Pack<i32>>(&u32::MAX, infer!()),
        &Pack(-1i32)
    );
    assert_eq!(cast_ref_alt::<u32, i32>(&u32::MAX, infer!()), &-1i32);
}

#[test]
fn try_cast_ref_alt_test() {
    assert_eq!(
        try_cast_ref_alt(&0, infer!((IsPod<u32>, IsPod<u16>))),
        Err(SizeMismatch)
    );
    assert_eq!(
        try_cast_ref_alt::<Pack<u32>, u32>(&Pack(0), infer!()),
        Err(TargetAlignmentGreaterAndInputNotAligned)
    );
    assert_eq!(
        try_cast_ref_alt::<u32, Pack<i32>>(&u32::MAX, infer!()),
        Ok(&Pack(-1i32))
    );
    assert_eq!(
        try_cast_ref_alt::<u32, i32>(&u32::MAX, infer!()),
        Ok(&-1i32)
    );
}

#[test]
fn cast_slice_alt_test() {
    must_panic(|| csa(&[0], infer!((IsPod<u32>, IsPod<u16>)))).unwrap();
    must_panic(|| csa::<Pack<u32>, u32>(&[Pack(0)], infer!())).unwrap();
    assert_eq!(
        csa::<u32, Pack<i32>>(&[u32::MAX, 1], infer!()),
        &[Pack(-1i32), Pack(1)][..]
    );
    assert_eq!(csa::<u32, i32>(&[u32::MAX, 2], infer!()), &[-1i32, 2][..]);
}

#[test]
fn try_cast_slice_alt_test() {
    assert_eq!(tcsa::<u32, u16>(&[0], Infer::INFER), Err(SizeMismatch));
    assert_eq!(
        tcsa::<Pack<u32>, u32>(&[Pack(0)], Infer::INFER),
        Err(TargetAlignmentGreaterAndInputNotAligned)
    );
    assert_eq!(
        tcsa::<u32, Pack<i32>>(&[u32::MAX, 3], Infer::INFER),
        Ok(&[Pack(-1i32), Pack(3)][..]),
    );
    assert_eq!(
        tcsa::<u32, i32>(&[u32::MAX, 4], Infer::INFER),
        Ok(&[-1i32, 4][..]),
    );
}
