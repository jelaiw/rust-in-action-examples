// use_value() takes ownership of the _val argument.
// The use_value() function is generic as it’s used in the next example.
fn use_value(_val: i32) {
}

fn main() {
    let a = 123;
    use_value(a);

    // It’s perfectly legal to access a after use_value() has returned.
    println!("{}", a);
}
