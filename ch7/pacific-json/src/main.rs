// Incorporates the serde_json crate and makes use of its macros, bringing the json! macro into scope.
#[macro_use]
extern crate serde_json;

fn main() {
    // json! takes a JSON literal and some Rust expressions to implement String values. It converts these into
    // a Rust value of type serde_json::Value, an enum that can represent every type within the JSON specification.
    let capitals = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa",
        "Niue": "Alofi",
        "Tonga": "Nuku'alofa",
        "Tuvalu": "Funafuti",
    });

    println!("The capital of Tonga is {}.", capitals["Tonga"]);
}
