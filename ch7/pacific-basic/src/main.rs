use std::collections::HashMap;

fn main() {
    // Type declarations of keys and values are not required here as these are inferred by the Rust compiler.
    let mut capitals = HashMap::new();

    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("Fiji", "Suva");
    capitals.insert("Kiribati", "South Tarawa");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku'alofa");
    capitals.insert("Tuvalu", "Funafuti");

    // HashMap implements Index, which allows for values to be retrieved via the square bracket indexing style.
    let tongan_capital = capitals["Tonga"];
    println!("The capital of Tonga is {}.", tongan_capital);
}
