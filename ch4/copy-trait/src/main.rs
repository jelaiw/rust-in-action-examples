#![allow(unused_variables)]

struct CubeSat {
    id: u64,
}

impl Copy for CubeSat { }

// Implementing Copy requires an implementation of Clone.
impl Clone for CubeSat {
    fn clone(&self) -> Self {
        // If desired, we can write out the creation of a new object ourselves...
        CubeSat {
            id: self.id,
        }
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

impl Copy for StatusMessage {}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}
