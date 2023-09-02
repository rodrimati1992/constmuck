// different size
const _: u8 = constmuck::pod_read_unaligned(&[0u8; 2]);
const _: u32 = constmuck::pod_read_unaligned(&[0u8; 2]);

#[cfg(feature = "debug_checks")]
const _: () = {
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    struct Foo(u8);

    unsafe impl constmuck::Contiguous for Foo {
        type Int = u8;
        const MIN_VALUE: u8 = 3;
        const MAX_VALUE: u8 = 2;
    }

    let _: Option<Foo> = constmuck::contiguous::from_integer(4);
};






fn main(){}