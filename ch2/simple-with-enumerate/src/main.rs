fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    // Because lines() returns an iterator, it can be chained with enumerate().
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
