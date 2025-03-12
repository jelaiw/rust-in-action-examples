use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), std::io::Error> {
    // File::open() returns Result<(), std::io::Error>.
    let _f:File = File::open("foo.txt")?;

    // "".parse::<Ipv6Addr>() returns Result<Ipv6Addr, std::net::AddrParseError>.
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
