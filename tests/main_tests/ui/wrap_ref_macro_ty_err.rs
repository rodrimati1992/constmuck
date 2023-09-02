use constmuck::wrapper::wrap_ref;
use std::num::Wrapping;

#[repr(transparent)]
struct Trans<T>(T);

unsafe impl<T> constmuck::TransparentWrapper<T> for Trans<T> {}

const fn foo(reff: &u8) -> &Wrapping<u16> {
    wrap_ref!(reff)
}

const fn different_outer(reff: &u8) -> &Wrapping<u8> {
    wrap_ref!(reff, Trans<u8>)
}

const fn different_inner_in_impl(reff: &u8) -> &Wrapping<u16> {
    wrap_ref!(reff, _, u8)
}

const fn different_inner_in_arg(reff: &u8) -> &Wrapping<u16> {
    wrap_ref!(reff, _, u16)
}


fn main(){}
