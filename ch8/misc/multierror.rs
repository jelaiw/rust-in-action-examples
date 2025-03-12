use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), std::io::Error> {
    // File::open() returns Result<(), std::io::Error>.
    // Doesn't File::open() return Result<File, std::io::Error>?
    // The ? operator is syntactic sugar for the try! macro.
    // Because File::open() returns std::io::Error, no conversion is necessary.
    let _f:File = File::open("foo.txt")?;

    // "".parse::<Ipv6Addr>() returns Result<Ipv6Addr, std::net::AddrParseError>.
    // The ? operator is syntactic sugar for the try! macro.
    // "".parse() presents ? with a std::net::AddrParseError. We don’t define how to convert
    // std::net::AddrParseError to std::io::Error, so the program fails to compile.
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
