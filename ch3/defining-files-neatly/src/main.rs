#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    // As File::new() is a completely normal function, we need to 
    // tell Rust that it will return a File from this function.
    fn new(name: &str) -> File {
        // File::new() does little more than encapsulate 
        // the object creation syntax, which is normal.
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    // This method sneaked in to deal with cases where we want
    // to simulate that a file has pre-existing data.
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    // Note f argument replaced with self.
    fn read(self: &File, save_to: &mut Vec<u8>,) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    // An explicit type needs to be provided as vec! and can’t infer
    // the necessary type through the function boundary.
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("f3.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    // Fields are private by default but can be accessed 
    // within the module that defines the struct.
    open(&mut f3);
    // Guess this means self has special meaning in Rust??
    let f3_length = f3.read(&mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text);
}
