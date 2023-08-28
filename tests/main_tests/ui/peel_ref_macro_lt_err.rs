use constmuck::wrapper::peel_ref;
use std::num::Wrapping;

const fn foo<'a>(reff: &'a Wrapping<u8>) -> &'static u8 {
    peel_ref!(reff)
}

const fn bar<'a>(reff: &'a Wrapping<u8>) -> &'static u8 {
    peel_ref!(reff, Wrapping<u8>)
}

const fn baz<'a>(reff: &'a Wrapping<u8>) -> &'static u8 {
    peel_ref!(reff, _, u8)
}

const fn qux<'a>(reff: &'a Wrapping<&'a u8>) -> &'a &'static u8 {
    peel_ref!(reff)
}


fn main(){}
