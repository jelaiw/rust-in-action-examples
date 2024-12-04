fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}

fn main() {
    let pw = "justok";
    let is_strong = is_strong(pw);
}
