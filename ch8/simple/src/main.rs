use std::error::Error;
use reqwest;

fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error> represents a trait object.
    let url = "http://www.rustinaction.com";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    print!("{}", content);

    Ok(())
}
