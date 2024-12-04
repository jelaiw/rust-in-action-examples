fn main() {
    let ptr = 42 as *const Vec<String>;

    unsafe {
        // https://doc.rust-lang.org/std/primitive.pointer.html#method.offset.
        let addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, addr);
    }
}
