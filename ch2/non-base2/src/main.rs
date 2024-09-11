fn main() {
    let three = 0b11; // The 0b prefix indicates binary (base 2) numerals.
    let thirty = 0o36; // The 0o prefix indicates octal (base 8) numerals.
    let three_hundred = 0x12C; // The 0x prefix indicates hexadecimal (base 16) numerals.

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
