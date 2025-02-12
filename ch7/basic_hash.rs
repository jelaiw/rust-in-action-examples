fn basic_hash(key: &str) -> u32 {
    // The .chars() iterator converts the string into a series of char values, each 4 bytes long.
    let first = key.chars()
        .next() // Returns an Option that’s either Some(char) or None for empty strings.
        // unwrap_or() behaves as unwrap() but provides a value rather than panicking when it encounters None.
        .unwrap_or('\0'); // If an empty string, provides NULL as the default.
    // Interprets the memory of first as an u32, even though its type is char.
    u32::from(first)
}

fn main() {
    println!("0x{:x}", basic_hash("foobar"));
}
