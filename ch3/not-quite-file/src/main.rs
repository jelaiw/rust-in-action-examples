// Relaxes compiler warnings while working through ideas.
#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // Makes a copy of the data here because save_to.append() shrinks the input Vec<T>.
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],

    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    // Converts Vec<u8> to String. Any bytes that are not valid UTF-8 are replaced with �.
    let text = String::from_utf8_lossy(&buffer); // https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
}
