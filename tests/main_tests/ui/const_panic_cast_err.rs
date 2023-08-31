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


// cast between ZSTs and non-ZSTs
const _: &[u8] = constmuck::cast_slice_alt(&[()]);
const _: &[()] = constmuck::cast_slice_alt(&[0u8]);

// OutputSliceWouldHaveSlop-y size cast
const _: &[[u8; 4]] = constmuck::cast_slice_alt(&[[2u8; 3]; 5]);

// different alignment
const _: &[u16] = {
    // Returning a reference with decreased alignment is fine.
    // Decreased element size is fine, so long as the slice size divides evenly.
    let _: &[u8] = constmuck::cast_slice_alt(&[0u16]);

    // returning a reference to type with a larger alignment is wrong
    constmuck::cast_slice_alt(&[[0u8; 2]; 3])
};


fn main(){}