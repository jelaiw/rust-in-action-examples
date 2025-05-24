use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    dbg!(secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        let num_bytes = std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        dbg!(num_bytes);

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            },
        }
    }
}
