#![allow(dead_code)]

// An enum’s variants are assumed to be public if the overall type is made public.
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    // File.data remains private if a third party were to import this crate via use.
    data: Vec<u8>,
    pub state: FileState,
}

impl File {
    // Even though the File struct is public, 
    // its methods must also be explicitly marked as public.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f7 = File::new("f7.txt");
    println!("{:?}", f7);
}
