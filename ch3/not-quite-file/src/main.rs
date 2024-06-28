// Relaxes compiler warnings while working through ideas.
#![allow(unused_variables)]

// Creates a type alias. The compiler won’t distinguish
// between String & File, but your source code will.
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// Relaxes a compiler warning about an unused function.
#[allow(dead_code)]
// The ! return type indicates to the Rust compiler that
// this function never returns.
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // A macro that crashes the program if it’s encountered.
    unimplemented!()
}

fn main() {
    // With the type declaration, File inherits all of String’s methods.
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}
