fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
        // No semicolon here. We want the result of this expression to feed into the outer scope.
        std::mem::transmute(a)
    };

    // Views the bits of a 42.42_f32 value as a decimal integer.
    println!("{}", frankentype);
    // {:032b} means to format as a binary via the std::fmt::Binary trait with 32 zeroes padded on the left.
    println!("{:032b}", frankentype);

    // https://doc.rust-lang.org/std/mem/fn.transmute.html
    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };
    println!("{}", b);
    // Confirms that the operation is symmetrical.
    assert_eq!(a, b);
}
