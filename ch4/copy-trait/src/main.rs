#![allow(unused_variables)]

struct CubeSat {
    id: u64,
}

impl Copy for CubeSat { }

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat {
            id: self.id,
        }
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
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
