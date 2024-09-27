fn main() {
    let n: f32 = 42.42;
    let bits: u32 = n.to_bits();

    println!("floating point number: {}", n);
    println!("raw bits: {:032b}", bits);
    
    let exponent_ = bits >> 23;
    println!("sans mantissa: {:032b}", exponent_);
    let exponent_ = exponent_ & 0xff;
    println!("sans sign bit: {:032b}", exponent_);

    let bias = 127i32;
    let exponent = (exponent_ as i32) - bias;
    println!("exponent: {}", exponent);
}
