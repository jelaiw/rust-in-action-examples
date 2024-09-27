fn main() {
    let n: f32 = 42.42;
    // Interpret bits of f32 as u32 to allow bit manipulation.
    // https://doc.rust-lang.org/std/primitive.f32.html#method.to_bits
    let bits: u32 = n.to_bits();
    let sign_bit = bits >> 31;

    println!("floating point number: {}", n);
    println!("raw bits: {:032b}", bits);
    println!("sign bit: {}", sign_bit);
}
