fn main() {
    let ptr = 42 as *const Vec<String>;

    unsafe {
        let addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, addr);
    }
}
