use constmuck::wrapper::peel_ref;
use std::num::Wrapping;

const fn bar(reff: &Wrapping<u8>) -> &u16 {
    peel_ref(reff)
}

fn main(){}
