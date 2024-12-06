fn main() {
    let a: i32 = 40; // 40 lives on the stack.
    let b: Box<i32> = Box::new(60); // 60 lives on the heap.

    println!("{} + {} = {}", a, b, a + *b); // To access 60, we need to dereference it.
}
