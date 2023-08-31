use super::test_utils::{must_panic, Pack};

use constmuck::{
    bytes_of, cast, cast_ref_alt, cast_slice_alt as csa, try_cast, try_cast_ref_alt,
    try_cast_slice_alt as tcsa,
    PodCastError::{
        OutputSliceWouldHaveSlop, SizeMismatch, TargetAlignmentGreaterAndInputNotAligned,
    },
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
    // PodCastError::TargetAlignmentGreaterAndInputNotAligned
    must_panic(|| csa::<Pack<u32>, u32>(&[Pack(0)])).unwrap();

    // PodCastError::SizeMismatch
    must_panic(|| csa::<(), u8>(&[()])).unwrap();

    // PodCastError::OutputSliceWouldHaveSlop
    must_panic(|| csa::<[u8; 3], [u8; 2]>(&[[0; 3]; 5])).unwrap();

    assert_eq!(
        csa::<u32, u16>(&[0x01020304u32.to_le()]),
        &[0x03_04, 0x01_02_u16][..]
    );

    assert_eq!(
        csa::<u32, Pack<i32>>(&[u32::MAX, 1]),
        &[Pack(-1i32), Pack(1)][..]
    );

    assert_eq!(csa::<u32, i32>(&[u32::MAX, 2]), &[-1i32, 2][..]);
}

#[test]
fn try_cast_slice_alt_test() {
    use std::num::Wrapping;
    assert_eq!(tcsa::<(), Wrapping<()>>(&[()]), Ok(&[Wrapping(())][..]));

    assert_eq!(tcsa::<u8, ()>(&[0u8]), Err(SizeMismatch));
    assert_eq!(tcsa::<(), u8>(&[()]), Err(SizeMismatch));

    // decreased alignment, slice's size evenly divides into new element size
    assert_eq!(
        tcsa::<u32, u16>(&[0x01020304u32.to_le()]),
        Ok(&[0x03_04, 0x01_02_u16][..])
    );

    assert_eq!(
        tcsa::<Pack<u32>, u32>(&[Pack(0)]),
        Err(TargetAlignmentGreaterAndInputNotAligned)
    );

    assert_eq!(tcsa::<u8, Pack<u32>>(&[0; 12]), Ok(&[Pack(0u32); 3][..]));
    assert_eq!(
        tcsa::<u8, Pack<u32>>(&[0; 13]),
        Err(OutputSliceWouldHaveSlop)
    );
    assert_eq!(
        tcsa::<u8, Pack<u32>>(&[0; 14]),
        Err(OutputSliceWouldHaveSlop)
    );
    assert_eq!(
        tcsa::<u8, Pack<u32>>(&[0; 15]),
        Err(OutputSliceWouldHaveSlop)
    );
    assert_eq!(tcsa::<u8, Pack<u32>>(&[0; 16]), Ok(&[Pack(0u32); 4][..]));

    assert_eq!(
        tcsa::<u32, Pack<i32>>(&[u32::MAX, 3]),
        Ok(&[Pack(-1i32), Pack(3)][..]),
    );
    assert_eq!(tcsa::<u32, i32>(&[u32::MAX, 4]), Ok(&[-1i32, 4][..]),);
}

#[test]
fn pod_read_unaligned_test() {
    use constmuck::pod_read_unaligned;

    let arr = [3, 5, 8, 13];
    assert_eq!(pod_read_unaligned::<u32>(&arr), u32::from_ne_bytes(arr));

    must_panic(|| pod_read_unaligned::<u32>(&[1; 0])).unwrap();
    must_panic(|| pod_read_unaligned::<u32>(&[1; 1])).unwrap();
    must_panic(|| pod_read_unaligned::<u32>(&[1; 2])).unwrap();
    must_panic(|| pod_read_unaligned::<u32>(&[1; 3])).unwrap();
    assert_eq!(pod_read_unaligned::<u32>(&[1; 4]), 0x01_01_01_01u32);
    must_panic(|| pod_read_unaligned::<u32>(&[1; 5])).unwrap();
    must_panic(|| pod_read_unaligned::<u32>(&[1; 6])).unwrap();
}

#[test]
fn pod_read_unaligned_large_alignment_test() {
    use constmuck::pod_read_unaligned as pru;

    #[derive(Copy, Clone)]
    #[repr(align(16))]
    struct Aligned(u128);

    unsafe impl bytemuck::Zeroable for Aligned {}
    unsafe impl bytemuck::Pod for Aligned {}

    let num = 0x01_02_03_04_05_06_07_08_09_10_11_12_13_14_15_u128;

    let aligned = pru::<Aligned>(&num.to_ne_bytes());
    assert_eq!(aligned.0, num);
}

#[test]
fn try_pod_read_unaligned_test() {
    use constmuck::try_pod_read_unaligned as tpru;

    let arr = [3, 5, 8, 13];
    assert_eq!(tpru::<u32>(&arr).unwrap(), u32::from_ne_bytes(arr));

    assert_eq!(tpru::<u32>(&[1; 0]).unwrap_err(), SizeMismatch);
    assert_eq!(tpru::<u32>(&[1; 1]).unwrap_err(), SizeMismatch);
    assert_eq!(tpru::<u32>(&[1; 2]).unwrap_err(), SizeMismatch);
    assert_eq!(tpru::<u32>(&[1; 3]).unwrap_err(), SizeMismatch);
    assert_eq!(tpru::<u32>(&[1; 4]).unwrap(), 0x01_01_01_01u32);
    assert_eq!(tpru::<u32>(&[1; 5]).unwrap_err(), SizeMismatch);
    assert_eq!(tpru::<u32>(&[1; 6]).unwrap_err(), SizeMismatch);
}
