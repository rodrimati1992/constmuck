This changelog is a summary of the changes made in each release.

# 0.1

### 0.1.0

Added `bytemuck` 1.7.2 as a dependency, with the `"min_const_generics"` feature.

Reexported the crate, `Contiguous`, `Pod`, `PodCastError`, `TransparentWrapper`, and `Zeroable` from `bytemuck`.

Defined the `contiguous` module with these items:
- reexport of `constmuck::ImplsContiguous`
- `FromInteger` struct
- `from_i8` function
- `from_i16` function
- `from_i32` function
- `from_i64` function
- `from_i128` function
- `from_isize` function
- `from_u8` function
- `from_u16` function
- `from_u32` function
- `from_u64` function
- `from_u128` function
- `from_usize`  function
- `into_integer` function

Defined the `copying` module with these items:
- reexport of `constmuck::ImplsCopy`
- `copy` functions
- `repeat` functions

Defined the `transmutable` module with these items:
- reexport of `constmuck::TransmutableInto`
- `transmute_ref` macro
- `transmute_ref` function
- `transmute_into` function
- `transmute_slice` function

Defined the `wrapper` module with these items:
- reexport of `constmuck::infer_tw;`
- reexport of `constmuck::ImplsTransparentWrapper;`
- `peel_ref` macro
- `wrap_ref` macro
- `peel` function
- `peel_ref` function
- `peel_slice` function
- `wrap` function
- `wrap_ref` function
- `wrap_slice` function

Defined these macros:
- `infer`
- `infer_tw`
- `map_bound`
- `type_size`

Defined these marker types:
- `ImplsContiguous`(not zero-sized)
- `ImplsCopy`
- `ImplsPod`
- `ImplsTransparentWrapper`
- `ImplsZeroable`
- `TransmutableInto`
- `TypeSize`(only zero-sized if it's `bounds` field is)

Defined the `Infer` trait

Defined these functions:
- `bytes_of`
- `cast`
- `cast_ref_alt`
- `cast_slice_alt`
- `try_cast`
- `try_cast_ref_alt`
- `try_cast_slice_alt`
- `zeroed`
- `zeroed_array`
