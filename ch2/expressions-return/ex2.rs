fn main() {
    let n = 654321;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };

    println!("{} is {}", n, description);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
