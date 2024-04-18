fn main() {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    println!("{}", absolute_difference);
    println!("{}", f32::EPSILON);
    assert!(absolute_difference <= f32::EPSILON);
}