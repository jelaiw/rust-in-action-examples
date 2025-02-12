fn basic_hash(key: &str) -> u32 {
    let first = key.chars()
        .next()
        .unwrap_or('\0');
    u32::from(first)
}

fn main() {
    println!("0x{:x}", basic_hash("foobar"));
}
