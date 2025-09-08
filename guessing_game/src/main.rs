use rand::Rng;
use std::cmp::Ordering;

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value needs to be between 1 and 100, got {}.", value);
        }
        Guess {
            value: value,
        }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    dbg!(secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        let _num_bytes = std::io::stdin().read_line(&mut guess).expect("Failed to read line");
//        dbg!(num_bytes);

        let guess:Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            },
        }
    }
}
