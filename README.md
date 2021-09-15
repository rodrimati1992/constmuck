[![Rust](https://github.com/rodrimati1992/constmuck/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/constmuck/actions)
[![crates-io](https://img.shields.io/crates/v/constmuck.svg)](https://crates.io/crates/constmuck)
[![api-docs](https://docs.rs/constmuck/badge.svg)](https://docs.rs/constmuck/*)


Const equivalents of many [`bytemuck`] functions,
and a few additional const functions.

`constmuck` uses `bytemuck`'s traits,
so any type that implements those traits can be used with the 
relevant functions from this crate.

The `*_alt` functions aren't exactly equivalent to the `bytemuck` ones,
each one describes how it's different.

# Examples

These examples use bytemuck's derives to show how users don't need to
write `unsafe` to use this crate,
and the [`konst`] crate to make writing the const functions easier.

### Contiguous

This example demonstrates constructing an enum from its representation.

```rust

use constmuck::{Contiguous, infer};

use konst::{array, try_opt};

fn main() {
    const COLORS: Option<[Color; 5]> = Color::from_array([3, 4, 1, 0, 2]);
    assert_eq!(
        COLORS,
        Some([Color::White, Color::Black, Color::Blue, Color::Red, Color::Green]),
    );

    const NONE_COLORS: Option<[Color; 4]> = Color::from_array([1, 2, 3, 5]);
    assert_eq!(NONE_COLORS, None);
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Contiguous, Copy, Clone)]
pub enum Color {
    Red = 0,
    Blue,
    Green,
    White,
    Black,
}

impl Color {
    pub const fn from_int(n: u8) -> Option<Self> {
        constmuck::contiguous::from_u8(n, infer!())
    }
    pub const fn from_array<const N: usize>(input: [u8; N]) -> Option<[Self; N]> {
        // `try_opt` returns from `from_array` on `None`,
        // because `konst::array::map` allows the passed-in expression
        // to return from the surrounding named function.
        Some(array::map!(input, |n| try_opt!(Self::from_int(n))))
    }
}


```

### Wrapper

This example demonstrates a type that wraps a `[T]`, constructed by reference.

```rust

use constmuck::TransparentWrapper;
use constmuck::infer_tw;

fn main() {
    const SLICE: &[u32] = &[3, 5, 8, 13, 21];
    const WRAPPER: &SliceWrapper<u32> = SliceWrapper::new(SLICE);

    const SUM: u64 = WRAPPER.sum();
    assert_eq!(SUM, 50);

    const FIRST_EVEN: Option<(usize, u32)> = WRAPPER.find_first_even();
    assert_eq!(FIRST_EVEN, Some((2, 8)));
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, TransparentWrapper)]
pub struct SliceWrapper<T>(pub [T]);

impl<T> SliceWrapper<T> {
    // Using `constmuck` allows safely defining this function as a `const fn`
    pub const fn new(reff: &[T]) -> &Self {
        constmuck::wrapper::wrap_ref!(reff, infer_tw!())
    }
}

impl SliceWrapper<u32> {
    pub const fn sum(&self) -> u64 {
        let mut sum = 0;
        konst::for_range!{i in 0..self.0.len() =>
            sum += self.0[i] as u64;
        }
        sum
    }
    pub const fn find_first_even(&self) -> Option<(usize, u32)> {
        konst::for_range!{i in 0..self.0.len() =>
            if self.0[i] % 2 == 0 {
                return Some((i, self.0[i]));
            }
        }
        None
    }
    
}


```


# Features

These are the features of this crate:

- `"derive"`(disabled by default):
enables `bytemuck`'s `"derive"` feature and reexports its derives.

- `"debug_checks"`(disabled by default):
Enables [`additional checks`](#additional-checks)

# Additional checks

The `"debug_checks"` feature enables additional checks,
all of which cause panics when it'd have otherwise been Undefined Behavior
(caused by unsound `unsafe impl`s or calling `unsafe` constructor functions).

##### Size checks

Functions that transmute values check that the value doesn't change size when transmuted.

Functions that transmute references check that referent (the `T` in `&T`)
doesn't change size when transmuted.

Macros that transmute references check that reference doesn't change size when transmuted
(ie: transmuting `&[u8]` to `&u8`).
Macros have weaker checking than functions because they allow references to `!Sized` types 
(eg: `str`, `[u8]`, `dyn Trait`),
if you're only casting references to `Sized` types it's better to use the function equivalents.


### Alignment checks

All the *functions* in the [`wrapper`] module check that the alignment of the 
`Inner` type parameter is the same as the `Outer` type parameter,
in addition to the size checks described in the previous section.

### Contiguous checks

The `from_*` functions in the [`contiguous`] module check that the 
`min_value` is less than the `max_value` of the passed-in `ImplsContiguous`.


# No-std support

`constmuck` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`constmuck` requires Rust 1.56.0, because it uses transmute inside const fns.


[`bytemuck`]: https://docs.rs/bytemuck/1.*/bytemuck/
[`konst`]: https://docs.rs/konst/*/konst/index.html
[`contiguous`]: https://docs.rs/constmuck/*/constmuck/contiguous/index.html
[`wrapper`]: https://docs.rs/constmuck/*/constmuck/wrapper/index.html

