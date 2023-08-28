use constmuck::wrapper::wrap_ref;
use std::num::Wrapping;

const fn bar(reff: &u16) -> &Wrapping<u8> {
    wrap_ref(reff)
}


fn main(){}
