fn main() {
    let x = (-42.0_f32).sqrt();
    println!("{}", x);
    assert_eq!(x, x);
}