fn main() {
    let n = 123456;
    let description = if is_even(n) {
        "even"
    }
    else {
        "odd"
    };

    println!("{} is {}", n, description);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
