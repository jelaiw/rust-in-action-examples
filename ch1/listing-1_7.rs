use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));

    println!("a: {a}, b: {b}, c: {c}, d: {:?}", d);
}