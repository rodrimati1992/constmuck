use super::test_utils::must_panic;

use constmuck::{contiguous, Contiguous};

#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum Tiny {
    N1 = -1,
    Z = 0,
    P1 = 1,
    P2 = 2,
}

unsafe impl Contiguous for Tiny {
    type Int = i8;

    const MIN_VALUE: i8 = -1;
    const MAX_VALUE: i8 = 2;
}

#[cfg(debug_assertions)]
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum Wrong {
    N1 = -1,
    Z = 0,
    P1 = 1,
}

#[cfg(debug_assertions)]
unsafe impl Contiguous for Wrong {
    type Int = i16;

    const MIN_VALUE: i16 = -1;
    const MAX_VALUE: i16 = 1;
}

#[cfg(debug_assertions)]
#[test]
fn swapped_limits() {
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    struct SwappedLimits(u8);

    unsafe impl Contiguous for SwappedLimits {
        type Int = i8;

        const MIN_VALUE: i8 = 2;
        const MAX_VALUE: i8 = 0;
    }

    must_panic(|| drop(contiguous::from_integer::<SwappedLimits>(1))).unwrap();
}

#[test]
fn convert_to_nonzero() {
    macro_rules! test_cases {
        ( $(($Int:ident, $NonZero:ident))* ) => ($({
            use std::num::$NonZero;

            let max = $Int::MAX;
            for n in [1, 2, 3, max - 3, max - 2, max - 1, max] {
                let nz = $NonZero::new(n).unwrap();

                assert_eq!(contiguous::from_integer(n), Some(nz));

                assert_eq!(contiguous::into_integer(nz), n);
            }
            let zero: $Int = 0;
            assert_eq!(contiguous::from_integer::<$NonZero>(zero), None);
        })*)
    }

    test_cases! {
        (u8, NonZeroU8)
        (u16, NonZeroU16)
        (u32, NonZeroU32)
        (u64, NonZeroU64)
        (u128, NonZeroU128)
        (usize, NonZeroUsize)
    }
}

#[test]
fn identity_conv() {
    macro_rules! test_cases {
        ( $($Int:ident)* ) => ($({
            let min = $Int::MIN;
            let max = $Int::MAX;
            for n in [min, min + 1, min + 2, 0, 1, 2, max - 3, max - 2, max - 1, max] {
                assert_eq!(contiguous::from_integer::<$Int>(n), Some(n));
            }
        })*)
    }

    test_cases! {i8 i16 i32 i64 i128 isize}
}
