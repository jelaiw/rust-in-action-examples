// Used as a type argument for a program’s various read_*() and write_*() methods.
use byteorder::LittleEndian;
// Traits that provide read_*() and write_*().
use byteorder::{ReadBytesExt, WriteBytesExt};
// As files support the ability to seek(), moving backward and forward to different
// positions, something is necessary to enable a Vec<T> to mock being a file.
// io::Cursor plays that role, enabling an in-memory Vec<T> to be file-like.
use std::io::Cursor;

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![];

    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;

    // Writes values to disk. These methods return io::Result, which we swallow here
    // as these won’t fail unless something is seriously wrong with the computer
    // that’s running the program.
    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);

    // Single byte types i8 and u8 don’t take an endianness parameter.
    w.write_i8(two).unwrap();
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one_ = r.read_u32::<LittleEndian>().unwrap();
    let two_ = r.read_i8().unwrap();
    let three_ = r.read_f64::<LittleEndian>().unwrap();

    (one_, two_, three_)
}

fn main() {
    let (one, two, three) = write_numbers_to_file();
    let (one_, two_, three_) = read_numbers_from_file();

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}
