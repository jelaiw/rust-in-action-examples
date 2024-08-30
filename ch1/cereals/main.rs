// Allows the println! macro to print the Cereal enum.
#[derive(Debug)]
// An enum (enumeration) is a type with a fixed number of legal variants.
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    // Initializes an empty vector of Cereal.
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    // Deletes grains and its contents.
    drop(grains);
    // Attempts to access the deleted value.
    println!("{:?}", grains);
}
