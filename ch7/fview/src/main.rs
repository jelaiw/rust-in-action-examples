use std::fs::File;
use std::io::prelude::*;
use std::env;

// Changing this constant changes the program’s output.
const BYTES_PER_LINE: usize = 8;

fn main() {
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
    let arg1 = env::args().nth(1);

    // https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    // https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }

        println!("");
        pos += BYTES_PER_LINE;
    }
}
