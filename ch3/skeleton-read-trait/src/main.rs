#![allow(unused_variables)]

#[derive(Debug)]
struct File;

// A trait block includes the type signatures of functions that implementors must comply with. 
trait Read {
    // The pseudo-type Self is a placeholder for the type that eventually implements Read.
    fn read(self: &Self, save_to: &mut Vec<u8>,) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>,) -> Result<usize, String> {
        // A simple stub value that complies with the type signature required.
        Ok(0)
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
