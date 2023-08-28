// different size
const _: u8 = constmuck::pod_read_unaligned(&[0u8; 2]);
const _: u32 = constmuck::pod_read_unaligned(&[0u8; 2]);




fn main(){}