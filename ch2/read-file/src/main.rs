use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // Creates a File object that requires a path argument and error handling
    // if the file does not exist. This program crashes if file is not present.
    let f = File::open("Cargo.lock").unwrap();
    let mut reader = BufReader::new(f);

    // Reuses a single String object over the lifetime of the program.
    let mut line = String::new();
    loop {
        // Because reading from disk can fail, we need to explicitly handle this.
        // In our case, errors crash the program.
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break
        }

        println!("{} ({} bytes long)", line, len);
        // Shrinks the String back to length 0, preventing lines from persisting
        // into the following ones.
        line.truncate(0);
    }
}
