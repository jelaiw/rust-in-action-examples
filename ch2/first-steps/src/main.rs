fn main() {
    let a = 10; // Types can be inferred by the compiler...
    let b: i32 = 20; // ...or declared by the programmer when creating variables.
    let c = 30i32; // Numeric types can include a type annotation in their literal form.
    // Numbers can include underscores, which are intended to increase readability and have no functional impact.
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);
}

// Type declarations are required when defining functions.
fn add(i: i32, j: i32) -> i32 {
    i + j // Functions return the last expression’s result so that return is not required.
}
