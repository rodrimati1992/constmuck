use super::test_utils::{must_panic, Pack};

use constmuck::{
    bytes_of, cast, cast_ref_alt, cast_slice_alt as csa, try_cast, try_cast_ref_alt,
    try_cast_slice_alt as tcsa,
    PodCastError::{SizeMismatch, TargetAlignmentGreaterAndInputNotAligned},
};

#[test]
fn bytes_of_test() {
    assert_eq!(bytes_of(&[-2i8, -1, 0, 1, 2]), &[254, 255, 0, 1, 2]);
    assert_eq!(bytes_of(&[0u8, 1, 2]), &[0, 1, 2]);
    assert_eq!(bytes_of(&123456789u32), &123456789u32.to_ne_bytes());
}

#[test]
fn cast_test() {
    must_panic(|| cast::<u32, i16>(0)).unwrap();

    assert_eq!(cast::<u32, i32>(u32::MAX), -1i32);
}

#[test]
fn try_cast_test() {
    assert_eq!(try_cast::<u32, u16>(0), Err(SizeMismatch));
    assert_eq!(try_cast::<u32, Pack<u32>>(1), Ok(Pack(1)));
    assert_eq!(try_cast::<u32, i32>(u32::MAX), Ok(-1i32));
}

#[test]
fn cast_ref_alt_test() {
    must_panic(|| cast_ref_alt::<u32, u16>(&0)).unwrap();
    must_panic(|| cast_ref_alt::<Pack<u32>, u32>(&Pack(0))).unwrap();
    assert_eq!(cast_ref_alt::<u32, Pack<i32>>(&u32::MAX), &Pack(-1i32));
    assert_eq!(cast_ref_alt::<u32, i32>(&u32::MAX), &-1i32);
}

#[test]
fn try_cast_ref_alt_test() {
    assert_eq!(try_cast_ref_alt::<u32, u16>(&0), Err(SizeMismatch));
    assert_eq!(
        try_cast_ref_alt::<Pack<u32>, u32>(&Pack(0)),
        Err(TargetAlignmentGreaterAndInputNotAligned)
    );
    assert_eq!(
        try_cast_ref_alt::<u32, Pack<i32>>(&u32::MAX),
        Ok(&Pack(-1i32))
    );
    assert_eq!(try_cast_ref_alt::<u32, i32>(&u32::MAX), Ok(&-1i32));
}

#[test]
fn cast_slice_alt_test() {
    must_panic(|| csa::<u32, u16>(&[0])).unwrap();
    must_panic(|| csa::<Pack<u32>, u32>(&[Pack(0)])).unwrap();
    assert_eq!(
        csa::<u32, Pack<i32>>(&[u32::MAX, 1]),
        &[Pack(-1i32), Pack(1)][..]
    );
    assert_eq!(csa::<u32, i32>(&[u32::MAX, 2]), &[-1i32, 2][..]);
}

#[test]
fn try_cast_slice_alt_test() {
    assert_eq!(tcsa::<u32, u16>(&[0]), Err(SizeMismatch));
    assert_eq!(
        tcsa::<Pack<u32>, u32>(&[Pack(0)]),
        Err(TargetAlignmentGreaterAndInputNotAligned)
    );
    assert_eq!(
        tcsa::<u32, Pack<i32>>(&[u32::MAX, 3]),
        Ok(&[Pack(-1i32), Pack(3)][..]),
    );
    assert_eq!(tcsa::<u32, i32>(&[u32::MAX, 4]), Ok(&[-1i32, 4][..]),);
}
