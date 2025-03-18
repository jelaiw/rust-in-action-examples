use std::fs::File;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(std::net::AddrParseError),
}

impl std::fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implements Display in terms of Debug via the "{:?}" syntax.
        write!(f, "{:?}", self)
    }
}

// Defers to default method implementations. The compiler will fill in the blanks.
impl std::error::Error for UpstreamError {}

fn main() -> Result<(), UpstreamError> {
    // https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err
    let _f:File = File::open("foo.txt").map_err(UpstreamError::IO)?;

    let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;

    Ok(())
}