fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    // See https://doc.rust-lang.org/std/fmt/#formatting-traits for refresher.
    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}
