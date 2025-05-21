fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    let num_bytes = std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    dbg!(num_bytes);

    println!("You guessed: {guess}");
}
