use std::rc::Rc;

#[derive(Debug)]
struct GroundStation;

fn main() {
    // Wrapping involves enclosing the GroundStation instance in a call to Rc::new().
    let base = Rc::new(GroundStation {});

    println!("{:?}", base);
}
