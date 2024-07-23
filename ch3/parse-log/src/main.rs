// Prints this enum to the screen via auto-generated code.
#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

// A convenient name for String for use in this crate’s context.
type Message = String;

// A function for parsing a line and converting it into semi-structured data.
fn parse_log(line: &str) -> (Event, Message) {
    // Vec<_> asks Rust to infer the elements’ type.
    // collect() consumes an iterator from line.splitn() and returns Vec.
    let parts: Vec<_> = line.splitn(2, " ").collect();

    // If line.splitn() doesn’t split log into two parts, returns an error.
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line))
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        // When we match a known event, returns structured data.
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        // If we don’t recognize the event type, returns the whole line.
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
