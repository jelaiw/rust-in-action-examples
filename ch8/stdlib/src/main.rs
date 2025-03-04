use std::io::prelude::Write;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Explicitly specifying the port (80) is required. TcpStream does not know that this will become a HTTP request.
    let host = "www.rustinaction.com:80";
    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?; // In many networking protocols, \r\n signifies a new line.

    conn.write_all(b"Host: www.rustinaction.com")?;
    conn.write_all(b"\r\n\r\n")?; // Two blank new lines signify end of request.

    // std::io::copy() streams bytes from a Reader to a Writer.
    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}
