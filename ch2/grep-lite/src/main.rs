use regex::Regex;

fn main() {
    // unwrap() unwraps a Result, crashing if an error occurs.
    let re = Regex::new("picture").unwrap();
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            // Some(T) is the positive case of an Option, meaning that re.find() was successful: it matches all values.
            Some(_) => println!("{}", line),
            // None is the negative case of an Option; () can be thought of as a null placeholder value here.
            None => (),
        }
    }
}
