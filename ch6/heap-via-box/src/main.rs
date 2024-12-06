// Brings manual drop() into local scope.
use std::mem::drop;

fn main() {
    // Allocates values on the heap.
    // Something that has been boxed lives on the heap, with a pointer to it on the stack.
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    // The unary *, the dereference operator, returns the value within the box.
    let result1 = *a + *b + *c;

    // Invokes drop(), freeing memory for other uses.
    drop(a);

    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
}
