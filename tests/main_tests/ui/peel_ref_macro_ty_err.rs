use constmuck::wrapper::peel_ref;
use std::num::Wrapping;

const fn foo<'a>(reff: &'a Wrapping<&'a u8>) -> &'a &'static u8 {
    peel_ref!(reff, constmuck::IsTW!())
}

fn main(){}
