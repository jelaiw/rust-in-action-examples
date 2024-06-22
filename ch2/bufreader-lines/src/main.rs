use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("Cargo.toml").unwrap();
    let reader = BufReader::new(f);

    // A subtle behavior change occurs here. BufReader::lines()
    // removes the trailing newline character from each line.
    for line_ in reader.lines() {
        // Unwraps the Result, but at the risk of crashing the
        // program if an error occurs.
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
