// Silences warnings related to FileState::Open not being used.
#![allow(dead_code)]

// Brings the std::fmt crate into local scope, making use of fmt::Result.
use std::fmt;
// Brings Display into local scope, avoiding the need to prefix it as fmt::Display.
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Sneakily, we can make use of write! to do the grunt work for us.
            // Strings already implement Display, so there’s little left for us to do.
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}
