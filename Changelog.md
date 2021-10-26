This changelog is a summary of the changes made in each release.

# 0.3.0

### 0.3.0

Renamed all `Impls*` types to `Is*`

Renamed `IsPod`'s fields, `impls_copy` to `is_copy` and `is_zeroable` to `impls_zeroable`

Renamed  `infer_tw` macro to `IsTW`

Renamed  `type_size` macro to `TypeSize`

Renamed `TypeSize::with_bound` to `with_bounds` and `set_bound` to `set_bounds`.

Renamed `bytes_of` to `byte_array_of`

Added these macros for constructing `Is*` types:
- `IsPod`: constructs `IsPod`
- `IsContiguous`: constructs `IsContiguous`
- `IsCopy`: constructs `IsCopy`
- `IsZeroable`: constructs `IsZeroable`

Added optional `$bound` type argument to `TypeSize` macro.

Swapped `TypeSize`'s' `B` and `T` type parameters

Added `IsTransparentWrapper::IDENTITY` associated constant

Added `Debug` impls for all types.

Added `"rust_latest_stable"` feature, to opt into using the latest "rust_*_*" feature
(only enabled once it's released in the stable channel).

Added `"rust_1_57"` feature

Removed `TransmutableInto`, and `transmutable` module, because they are leaky abstractions.

Removed the `into_inner` and `from_inner` fields in `IsTransparentWrapper`.

Changed `contiguous::into_integer` to take `IsContiguous` by reference.

Swapped `Inner` and `Outer` type parameters of `wrap*` and `peel*` functions to have the same order as `IsTransparentWrapper`

Made all type parameters of `Is*`  types and `TypeSize` invariant, just in case that it's unsound for them to be covariant.

Removed the `"debug_checks"` feature, replacing it with the built-in `debug_assertions` flag.

Added `const_panic` 0.1.0 as a dependency, enabled by the `"rust_1_57"` feature.

# 0.2

### 0.2.0

Changed `ImplsCopy` bound to `Pod`

Added `join` methods to both `TransmutableInto` and `ImplsTransparentWrapper`

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

Added "debug_checks" feature, to check many things when they'd otherwise been undefined behavior.
