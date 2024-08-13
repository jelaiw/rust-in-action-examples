#![allow(dead_code)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    // &self indicates that GroundStation.send() only requires a read-only reference to self.
    // The recipient takes a mutable borrow (&mut) of the CubeSat instance, and msg takes
    // full ownership of its Message instance.
    fn send(&self, to: &mut CubeSat, msg: Message) {
        // Ownership of the Message instance transfers from msg to messages.push() as a local variable.
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    // https://doc.rust-lang.org/stable/core/option/index.html
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox {
            messages: vec![],
        },
    };

    println!("t0: {:?}", sat_a);

    // We don’t have a completely ergonomic way to create Message instances yet.
    // Instead, we’ll take advantage of the String.from() method that converts
    // &str to String (aka Message).
    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
