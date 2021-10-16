use constmuck::wrapper::wrap_ref;
use std::num::Wrapping;

const fn foo<'a>(reff: &'a u8) -> &'static Wrapping<u8> {
    wrap_ref(reff, constmuck::IsTW!())
}

fn main(){}
