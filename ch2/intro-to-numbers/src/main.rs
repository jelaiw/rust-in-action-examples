fn main() {
    let twenty = 20; // Rust infers a type on your behalf if you don’t supply one...
    let twenty_one: i32 = 21; // ..which is done by adding type annotations
    let twenty_two = 22i32; // ..or type suffixes.

    let sum = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, sum);

    let one_million: i64 = 1_000_000; // Underscores increase readability and are ignored by the compiler.
    println!("{}", one_million.pow(2)); // Numbers have methods.

    // Creates an array of numbers, which must all be the same type.
    let forty_twos = [
        42.0, // Floating-point literals without an explicit type annotation become 32-bit or 64-bit, depending on context.
        42f32, // Floating-point literals can also have type suffixes...
        42.0_f32, // ..and optional underscores.
    ];

    // Format is a bit of a head scratcher because a width of 2 with '0' as the fill character isn't doing much here.
    // Maybe :.1 to print 42.0 (the floating point value that was bound) instead of 42?
    // See https://doc.rust-lang.org/std/fmt/#formatting-parameters.
    println!("{:02}", forty_twos[0]);
}
