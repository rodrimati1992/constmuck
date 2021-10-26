use constmuck::wrapper::{peel_ref, wrap_ref};
use std::num::Wrapping;

const fn peel_ref_fn_lt<'a>(reff: &'a Wrapping<u8>) -> &'a u8 {
    peel_ref(reff, constmuck::IsTW!())
}

const fn peel_ref_fn_ty<'a>(reff: &'a Wrapping<&'a u8>) -> &'a &'a u8 {
    peel_ref(reff, constmuck::IsTW!())
}

const fn peel_ref_macro_lt<'a>(reff: &'a Wrapping<u8>) -> &'a u8 {
    peel_ref!(reff, constmuck::IsTW!())
}

const fn peel_ref_macro_ty<'a>(reff: &'a Wrapping<&'a u8>) -> &'a &'a u8 {
    peel_ref!(reff, constmuck::IsTW!())
}

const fn wrap_ref_fn_lt<'a>(reff: &'a u8) -> &'a Wrapping<u8> {
    wrap_ref(reff, constmuck::IsTW!())
}

const fn wrap_ref_fn_ty<'a>(reff: &'a &'a u8) -> &'a Wrapping<&'a u8> {
    wrap_ref(reff, constmuck::IsTW!())
}

const fn wrap_ref_macro_lt<'a>(reff: &'a u8) -> &'a Wrapping<u8> {
    wrap_ref!(reff, constmuck::IsTW!())
}

const fn wrap_ref_macro_ty<'a>(reff: &'a &'a u8) -> &'a Wrapping<&'a u8> {
    wrap_ref!(reff, constmuck::IsTW!())
}

fn main(){}
