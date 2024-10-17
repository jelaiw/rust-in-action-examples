fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    // Aligns the input byte n to 32 bits, then increases its value
    // by shifting its bits 15 places to the left.
    let large_n = (n as u32) << 15;
    // Takes a bitwise OR, merging the base with the input byte.
    let f32_bits = base | large_n;
    // Interprets f32_bits (which is type u32) as an f32.
    let m = f32::from_bits(f32_bits);
    // Normalizes the output range.
    2.0 * ( m - 0.5 )
}

fn main() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
