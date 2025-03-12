use std::fs::File;
use std::net::Ipv6Addr;
use std::error::Error;

// A trait object, Box<dyn Error>, represents any type that implements std::error::Error.
fn main() -> Result<(), Box<dyn Error>> {
    // File::open() returns Result<(), std::io::Error>.
    // Doesn't File::open() return Result<File, std::io::Error>?
    let _f:File = File::open("foo.txt")?;

    // "".parse::<Ipv6Addr>() returns Result<Ipv6Addr, std::net::AddrParseError>.
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
