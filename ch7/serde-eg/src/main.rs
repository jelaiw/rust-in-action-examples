use serde_derive::Serialize;
use serde_json::to_string as to_json;
use serde_cbor::to_vec as to_cbor;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();

    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);

    println!("json (as UTF-8):\n{}\n", String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor (as UTF-8):\n{}\n", String::from_utf8_lossy(&as_cbor));
}
