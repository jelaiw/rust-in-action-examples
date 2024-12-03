fn main() {
    let a: i64 = 42;
    // Casts a reference to the variable a (&a) to a constant raw pointer i64 (*const i64).
    let a_ptr = &a as *const i64;

    println!("a: {} ({:p})", a, a_ptr);
}
