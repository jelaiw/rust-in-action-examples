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

impl From<std::io::Error> for UpstreamError {
    fn from(error: std::io::Error) -> Self {
        UpstreamError::IO(error)
    }
}

impl From<std::net::AddrParseError> for UpstreamError {
    fn from(error: std::net::AddrParseError) -> Self {
        UpstreamError::Parsing(error)
    }
}

// Defers to default method implementations. The compiler will fill in the blanks.
impl std::error::Error for UpstreamError {}

fn main() -> Result<(), UpstreamError> {
    let _f:File = File::open("foo.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}