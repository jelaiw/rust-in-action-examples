static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;

    // The {:p} syntax asks Rust to format the variable as a pointer and prints
    // the memory address that the value points to.
    // See https://doc.rust-lang.org/std/fmt/#syntax for further detail.
    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}
