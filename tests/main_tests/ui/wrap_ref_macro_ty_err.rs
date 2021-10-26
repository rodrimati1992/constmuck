use constmuck::wrapper::wrap_ref;
use std::num::Wrapping;

const fn foo<'a>(reff: &'a &'a u8) -> &'a Wrapping<&'static u8> {
    wrap_ref!(reff, constmuck::IsTW!())
}

fn main(){}
