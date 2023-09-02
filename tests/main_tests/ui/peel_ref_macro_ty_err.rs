use constmuck::wrapper::peel_ref;
use std::num::Wrapping;

#[repr(transparent)]
struct Trans<T>(T);

unsafe impl<T> constmuck::TransparentWrapper<T> for Trans<T> {}

const fn foo(reff: &Wrapping<u8>) -> &u16 {
    peel_ref!(reff)
}

const fn different_outer(reff: &Wrapping<u8>) -> &u8 {
    peel_ref!(reff, Trans<u8>)
}

const fn different_inner_than_impld(reff: &Wrapping<u8>) -> &u16 {
    peel_ref!(reff, _, u16)
}

const fn different_inner_than_returned(reff: &Wrapping<u8>) -> &u16 {
    peel_ref!(reff, _, u8)
}

fn main(){}
