use constmuck::wrapper::peel_ref;
use std::num::Wrapping;

const fn foo<'a>(reff: &'a Wrapping<u8>) -> &'static u8 {
    peel_ref(reff, constmuck::IsTW!())
}

fn main(){}
