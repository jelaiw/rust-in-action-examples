fn main() {
    let n: f32 = 42.42;
    let bits: u32 = n.to_bits();

    println!("floating point number: {}", n);
    println!("raw bits: {:032b}", bits);
    
    // Right shift to overwrite mantissa.
    let exponent_ = bits >> 23;
    println!("sans mantissa: {:032b}", exponent_);
    // Flip the sign bit to make the mask step more interesting.
    let exponent_ = exponent_ | 0x100;
    println!("flip sign bit: {:032b}", exponent_);
    // Filter out sign bit with AND mask.
    let exponent_ = exponent_ & 0xff;
    println!("sans sign bit: {:032b}", exponent_);

    // Interpret the remaining bits as a signed integer and subtract the bias as defined by the standard.
    let bias = 127i32;
    let exponent = (exponent_ as i32) - bias;
    println!("exponent: {}", exponent);
}
