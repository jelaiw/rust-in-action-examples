fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {
        println!("Ten is less than one hundred.")
    }
    
    let x = 300_i32 as i8;
    println!("x = {}", x);
}
