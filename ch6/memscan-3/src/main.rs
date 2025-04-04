// Creates a global static, which is a global variable in Rust programs.
static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    // Creates a local variable within noop() so that something outside of main() has a memory address.
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_int = Box::new(789);
    let boxed_str = Box::new('b'); // Note, this is actually a boxed char.
    let fn_int = noop();

    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int:    {:p}", fn_int);
}
