fn main() {
    let x: f32 = 1.0 / 0.0;
    println!("{}", x);
    println!("{}", x.is_finite());
    println!("{}", x.is_nan());
    assert!(x.is_finite());
}