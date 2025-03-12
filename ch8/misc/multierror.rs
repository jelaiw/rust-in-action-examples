use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), std::io::Error> {
    let _f:File = File::open("foo.txt")?;

    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
