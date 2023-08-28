// different size
const _: u16 = constmuck::cast(0u8);
const _: u8 = constmuck::cast(0u16);


// different alignment
const _: &u16 = {
    // returning a reference with decreased alignment is fine 
    let _: &[u8; 2] = constmuck::cast_ref_alt(&0u16);

    // returning a reference to type with a larger alignment is wrong
    constmuck::cast_ref_alt(&[0u8; 2])
};

// different size
const _: &u8 = constmuck::cast_ref_alt(&[0u8; 2]);
const _: &[u8; 2] = constmuck::cast_ref_alt(&0u8);


// different element size
const _: &[u8] = constmuck::cast_slice_alt(&[[0u8; 2]; 3]);
const _: &[[u8; 2]] = constmuck::cast_slice_alt(&[0u8; 3]);

// different alignment
const _: &[u16] = {
    // returning a reference with decreased alignment is fine 
    let _: &[[u8; 2]] = constmuck::cast_slice_alt(&[0u16]);

    // returning a reference to type with a larger alignment is wrong
    constmuck::cast_slice_alt(&[[0u8; 2]; 3])
};


fn main(){}