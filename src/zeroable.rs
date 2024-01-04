use bytemuck::Zeroable;

/// Constructs a zero-initialized `T`,
/// safe equivalent to [`std::mem::zeroed::<T>()`](core::mem::zeroed).
///
/// # Panics
///
/// If the `"rust_1_75"` feature is disabled,
/// then this function panics when `size_of::<T>()` is larger than `1_048_576` bytes.
///
/// # Example
///
/// ```rust
/// use constmuck::zeroed;
///
/// const BYTES: [u8; 4] = zeroed();
/// const CHARS: [char; 4] = zeroed();
///
/// assert_eq!(BYTES, [0, 0, 0, 0]);
/// assert_eq!(CHARS, ['\0', '\0', '\0', '\0']);
///
/// ```
#[cfg_attr(feature = "rust_1_75", inline(always))]
pub const fn zeroed<T: Zeroable>() -> T {
    #[cfg(feature = "rust_1_75")]
    {
        unsafe { core::mem::zeroed() }
    }

    #[cfg(not(feature = "rust_1_75"))]
    {
        macro_rules! last {
            ($curr:tt) => { $curr };
            ($curr:tt $($rem:tt)+) => { last!($($rem)+) };
        }

        let size = core::mem::size_of::<T>();

        macro_rules! zeroed_sizes {
            ($($size_bound:expr),* $(,)*) => {
                $(
                    if size <= $size_bound {
                        // safety:
                        // `IsZeroable<T>` guarantees that it's valid to produce a `T`
                        // that is represented as all zero bytes.
                        //
                        // `size_of::<T>() <= $size_bound` holds
                        unsafe {
                            zeroed_with_size::<T, $size_bound>()
                        }
                    }
                )else*
                else {
                    use crate::const_panic::{FmtArg as FA, PanicVal as PV};

                    crate::const_panic::concat_panic(&[&[
                        PV::write_str("\n`constmuck::zeroed` can only instantiate types up to "),
                        PV::from_usize(last!($($size_bound)*), FA::DEBUG),
                        PV::write_str(" bytes large, but the type is "),
                        PV::from_usize(size, FA::DEBUG),
                        PV::write_str(" bytes large"),
                    ]])
                }
            };
        }

        zeroed_sizes! {
            64,
            256,
            1024,
            4096,
            16384,
            65536,
            262144,
            1048576,
        }
    }
}

// # Safety
//
// `std::mem::size_of::<T>()` must be less than or equal to `SJZE`.
#[cfg(not(feature = "rust_1_75"))]
const unsafe fn zeroed_with_size<T: Zeroable, const SIZE: usize>() -> T {
    core::mem::ManuallyDrop::into_inner(
        crate::__priv_utils::Transmuter {
            from: core::mem::ManuallyDrop::new([0u8; SIZE]),
        }
        .to,
    )
}
