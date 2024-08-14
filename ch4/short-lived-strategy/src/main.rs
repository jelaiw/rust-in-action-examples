#![allow(dead_code)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

struct Mailbox {
    messages: Vec<Message>,
}


#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation;

impl Mailbox {
    // Mailbox.post() requires mutable access to itself and ownership over a Message.
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    // Mailbox.deliver() requires a shared reference to a CubeSat to pull out its id field.
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                // When we find a message, returns early with the Message wrapped in Some per the Option type.
                return Some(msg);
            }
        }
        // When no messages are found, returns None.
        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
        }
    }

    // Calls Mailbox.post() to send messages, yielding ownership of a Message.
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    // Calls Mailbox.deliver() to receive messages, gaining ownership of a Message.
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        return mailbox.deliver(&self);
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![0, 1, 2]
}

fn main() {
    let mut mailbox = Mailbox {
        messages: vec![],
    };

    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let msg = Message {
            to: sat_id,
            content: String::from("hello")
        };
        base.send(&mut mailbox, msg);
    }

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mailbox);

        println!("{:?}: {:?}", sat, msg);
    }
}
