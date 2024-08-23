fn greet_world() {
    // The exclamation mark indicates the use of a macro.
    println!("Hello, world!");
    // Assignment in Rust, more properly called variable binding, uses the let keyword.
    let southern_germany = "Grüß Gott!";
    // Unicode support is provided out of the box.
    let japan = "ハロー・ワールド";
    // Array literals use square brackets.
    let regions = [southern_germany, japan];
    // Many types can have an iter() method to return an iterator.
    for region in regions.iter() {
        // The ampersand “borrows” region for read-only access.
        println!("{}", &region);
    }
}
fn main() {
    greet_world()
}
