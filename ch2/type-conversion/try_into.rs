// Not strictly necessary now, see Rust 2021 prelude. Book published in 2021.
// https://doc.rust-lang.org/std/prelude/index.html
// https://doc.rust-lang.org/std/convert/trait.TryInto.html
use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // try_into() returns a Result type that provides access to the conversion attempt.
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.")
    }
}
