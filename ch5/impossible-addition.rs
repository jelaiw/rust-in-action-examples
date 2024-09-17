fn main() {
    let (a, b) = (200, 200);
    // Without the type declaration, Rust won’t assume that you’re trying to create an impossible situation.
    let c: u8 = a + b;

    println!("{} + {} = {}", a, b, c);
}
