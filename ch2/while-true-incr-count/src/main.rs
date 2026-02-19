use std::time::{Duration, Instant};
fn main() {
    let mut count = 0;
    // https://doc.rust-lang.org/std/time/struct.Duration.html
    let time_limit = Duration::new(1, 0); // Creates a Duration that represents 1 second.
    // https://doc.rust-lang.org/std/time/struct.Instant.html
    let start = Instant::now(); // Accesses time from the system’s clock.

    // An Instant minus an Instant returns a Duration.
    // https://doc.rust-lang.org/std/time/struct.Instant.html#impl-Sub-for-Instant
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}
