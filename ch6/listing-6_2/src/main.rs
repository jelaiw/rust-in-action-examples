use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size: {:?} bytes", size_of::<usize>());
    println!("  value: {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("  location: {:p}", &b);
    println!("  size: {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to: {:p}", b);
    println!();

    println!("B (an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size: {:?} bytes", size_of::<[u8; 10]>());
    println!("  value: {:?}", B);
    println!();
}
