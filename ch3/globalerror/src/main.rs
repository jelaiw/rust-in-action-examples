use rand::random;

// A global variable, static mut (or mutable static), with a
// static lifetime that’s valid for the life of the program.
static mut ERROR: isize = 0;

// Creates a zero-sized type to stand in for a struct
// while we’re experimenting.
struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // Returns true one out of eight times this function is called.
    if random() && random() && random() {
        // Accessing and modifying static mut variables requires
        // the use of an unsafe block. This is Rust’s way of
        // disclaiming all responsibility.
        unsafe {
            ERROR = 1;
        }
    }
    0
}

// Keeping buffer mutable for consistency with other code
// even though it isn’t touched here.
#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    // Accessing static mut variables is an unsafe operation.
    unsafe {
        // Error checking relies on the convention that 0 means no error.
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}
