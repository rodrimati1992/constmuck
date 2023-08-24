use bytemuck::Zeroable;

/// Constructs a zero-initialized `T`,
/// equivalent to [`std::mem::zeroed::<T>()`](core::mem::zeroed).
///
/// This function requires that `T` implements [`Zeroable`].
///
/// # Panics
///
/// this function panics if
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
///
/// [`Zeroable`]: trait@Zeroable
pub const fn zeroed<T: Zeroable>() -> T {
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
                    // that is represented a all zero bytes.
                    //
                    // `size_of::<T>() <= $size_bound` holds
                    unsafe {
                        zeroed_with_size::<T, $size_bound>()
                    }
                }
            )else*
            else {
                use crate::const_panic::{concat_panic, FmtArg as FA, PanicVal as PV};

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

// # Safety
//
// `std::mem::size_of::<T>()` must be less than or equal to `SJZE`.
const unsafe fn zeroed_with_size<T: Zeroable, const SIZE: usize>() -> T {
    __priv_transmute! {[u8; SIZE], T, [0u8; SIZE]}
}
