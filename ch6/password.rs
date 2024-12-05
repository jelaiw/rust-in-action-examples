// fn x<T: AsRef<str>>(a: T) reads as "function x takes argument a of
// type T, where T implements AsRef<str>".
//fn is_strong<T: AsRef<str>>(password: T) -> bool {
//    password.as_ref().len() > 5
//}

fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}

fn main() {
    let pw = "justok";
    let is_strong = is_strong(pw);
}
