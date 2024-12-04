fn is_strong(password: String) -> bool {
    password.len() > 5
}

fn main() {
    let pw = "justok";
    let is_strong = is_strong(pw);
}
