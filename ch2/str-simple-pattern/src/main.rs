fn main() {
    let search_term = "picture";
    // Multilined strings do not require special syntax. The \ character on line 4 escapes the new line.
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    // lines() returns an iterator over quote where each iteration is a line of text. Rust uses each
    // operating system’s conventions on what constitutes a new line.
    // https://doc.rust-lang.org/std/primitive.str.html#method.lines
    for line in quote.lines() {
        // https://doc.rust-lang.org/std/primitive.str.html#method.contains
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}
