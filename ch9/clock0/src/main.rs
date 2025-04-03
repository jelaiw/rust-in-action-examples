use chrono::Local;

fn main() {
    // https://docs.rs/chrono/latest/chrono/offset/struct.Local.html#method.now
    let now = Local::now();
    println!("{}", now);
}
