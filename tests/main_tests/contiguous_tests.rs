use super::test_utils::must_panic;

use constmuck::{
    contiguous::{self, FromInteger},
    infer, Contiguous, ImplsContiguous,
};

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

#[cfg(feature = "debug_checks")]
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum Wrong {
    N1 = -1,
    Z = 0,
    P1 = 1,
}

#[cfg(feature = "debug_checks")]
unsafe impl Contiguous for Wrong {
    type Int = i16;

    const MIN_VALUE: i16 = -1;
    const MAX_VALUE: i16 = 1;
}

#[test]
fn contiguous_accessors() {
    {
        let ic = ImplsContiguous::<Tiny, i8>::NEW;
        assert_eq!(ic.min_value(), &-1);
        assert_eq!(ic.max_value(), &2);
    }
    {
        let ic = ImplsContiguous::<u32, u32>::NEW;
        assert_eq!(ic.min_value(), &0);
        assert_eq!(ic.max_value(), &u32::MAX);
    }
}

#[test]
fn custom_type_tests() {
    for outside in (i8::MIN..=-2).chain(3..=i8::MAX) {
        assert_eq!(contiguous::from_i8::<Tiny>(outside, infer!()), None);
        assert_eq!(FromInteger::<Tiny, i8>(outside, infer!()).call(), None);
    }

    for variant in [Tiny::N1, Tiny::Z, Tiny::P1, Tiny::P2] {
        assert_eq!(contiguous::from_i8(variant as i8, infer!()), Some(variant));
        assert_eq!(FromInteger(variant as i8, infer!()).call(), Some(variant));
    }

    #[cfg(feature = "debug_checks")]
    unsafe {
        macro_rules! make_ic {
            ($ty:ty) => {
                ImplsContiguous::<Wrong, $ty>::new_unchecked((Wrong::N1 as $ty).min(0), 1)
            };
        }
        // making sure to test u8 since `from_u8` is manually written,
        // while the rest are macro generated
        must_panic(|| drop(contiguous::from_u8::<Wrong>(0, make_ic!(u8)))).unwrap();
        must_panic(|| drop(contiguous::from_i8::<Wrong>(0, make_ic!(i8)))).unwrap();
        must_panic(|| drop(contiguous::from_i16::<Wrong>(0, make_ic!(i16)))).unwrap();
        must_panic(|| drop(contiguous::into_integer(Wrong::Z, make_ic!(i16)))).unwrap();
    }
}

#[test]
fn convert_to_nonzero() {
    macro_rules! test_cases {
        (
            $(($Int:ident, $NonZero:ident, $from_fn:ident))*
        ) => ($({
            use std::num::$NonZero;

            use constmuck::contiguous::$from_fn;

            let max = $Int::MAX;
            for n in [1, 2, 3, max - 3, max - 2, max - 1, max] {
                let nz = $NonZero::new(n).unwrap();

                assert_eq!($from_fn(n, infer!()), Some(nz));
                assert_eq!(FromInteger(n, infer!()).call(), Some(nz));

                assert_eq!(contiguous::into_integer(nz, infer!()), n);
            }
            let zero: $Int = 0;
            assert_eq!($from_fn::<$NonZero>(zero, infer!()), None);
            assert_eq!(FromInteger::<$NonZero, $Int>(zero, infer!()).call(), None);
        })*)
    }

    test_cases! {
        (u8, NonZeroU8, from_u8)
        (u16, NonZeroU16, from_u16)
        (u32, NonZeroU32, from_u32)
        (u64, NonZeroU64, from_u64)
        (u128, NonZeroU128, from_u128)
        (usize, NonZeroUsize, from_usize)
    }
}

#[test]
fn identity_conv() {
    macro_rules! test_cases {
        (
            $(($Int:ident, $from_fn:ident))*
        ) => ($({
            use constmuck::contiguous::$from_fn;

            let min = $Int::MIN;
            let max = $Int::MAX;
            for n in [min, min + 1, min + 2, 0, 1, 2, max - 3, max - 2, max - 1, max] {
                assert_eq!($from_fn::<$Int>(n, infer!()), Some(n));
                assert_eq!(FromInteger::<$Int, $Int>(n, infer!()).call(), Some(n));
            }
        })*)
    }

    test_cases! {
        (i8, from_i8)
        (i16, from_i16)
        (i32, from_i32)
        (i64, from_i64)
        (i128, from_i128)
        (isize, from_isize)
    }
}
