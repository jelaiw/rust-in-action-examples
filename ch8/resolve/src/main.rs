use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{App, Arg};
use rand;
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::*;

fn main() {
    let app = App::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(Arg::with_name("dns-server").short("s").default_value("1.1.1.1"))
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();

    let domain_name_raw = app.value_of("domain-name").unwrap();
    let domain_name = Name::from_ascii(&domain_name_raw).unwrap();

    let dns_server_raw = app.value_of("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw).parse().expect("invalid address");

    // Creates a Vec<T> with length 0 and capacity 512.
    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    // Creates a Vec<T> with length 512 and capacity 512.
    // recv_from() checks that this buffer has sufficient space with length field.
    let mut response_as_bytes: Vec<u8> = vec![0; 512];
    println!("{} {}", request_as_bytes.len(), response_as_bytes.len());

    // Message represents a DNS message, which is a container for queries and other information such as answers.
    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        // Specifies that this is a DNS query, not a DNS answer.
        // Both have the same representation over the wire, but not in Rust’s type system.
        .set_message_type(MessageType::Query)
        // Multiple queries can be included in the same message.
        .add_query(
            // The equivalent type for IPv6 addresses is AAAA.
            Query::query(domain_name, RecordType::A)
        )
        .set_op_code(OpCode::Query)
        // Requests that the DNS server asks other DNS servers if it doesn’t know the answer.
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    // 0.0.0.0:0 means listen to all addresses on a random port. The OS selects the actual port.
    let localhost = UdpSocket::bind("0.0.0.0:0")
        .expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost.send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");

    let (_amt, _remote) = localhost.recv_from(&mut response_as_bytes)
        .expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes)
        .expect("unable to parse response");

    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let ip = resource.to_ip_addr().expect("invalid IP address received");
            println!("{}", ip.to_string());
        }
    }
}
