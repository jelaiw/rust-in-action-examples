struct Demo {
    a: i32,
}

// use_value() takes ownership of _val.
fn use_value(_val: Demo) {
}

fn main() {
    let demo = Demo { a: 123 };
    use_value(demo);

    // It’s illegal to access demo.a, even after use_value() has returned.
    println!("{}", demo.a);
}
