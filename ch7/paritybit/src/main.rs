// Takes a byte slice as the bytes argument and returns a single byte as output. This function could have easily
// returned a bool value, but returning u8 allows the result to bit shift into some future desired position.
fn parity_bits(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;

    for byte in bytes {
        // All of Rust’s integer types come equipped with count_ones() and count_zeros() methods.
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }

    // There are plenty of methods to optimize this function. One fairly simple approach is to hard code a
    // const [u8; 256] array of 0s and 1s, corresponding to the intended result, then index that array with each byte.
    (n_ones % 2 == 0) as u8
}

fn main() {
    let abc = b"abc";
    println!("input: {:?}", abc);
    println!("output: {:08b}", parity_bits(abc));
    println!();

    let abcd = b"abcd";
    println!("input: {:?}", abcd);
    println!("output: {:08b}", parity_bits(abcd));
    println!();
}
