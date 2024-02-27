```sh
$ cargo r
   Compiling letters v0.1.0 (/workspaces/rust-in-action-examples/ch1/letters)
error[E0382]: borrow of moved value: `letters`
   --> src/main.rs:8:9
    |
2   |     let mut letters = vec![
    |         ----------- move occurs because `letters` has type `Vec<&str>`, which does not implement the `Copy` trait
...
6   |     for letter in letters {
    |                   ------- `letters` moved due to this implicit call to `.into_iter()`
7   |         println!("{}", letter);
8   |         letters.push(letter.clone());
    |         ^^^^^^^ value borrowed here after move
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `letters`
   --> /usr/local/rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:268:18
    |
268 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
help: consider iterating over a slice of the `Vec<&str>`'s content to avoid moving into the `for` loop
    |
6   |     for letter in &letters {
    |                   +

For more information about this error, try `rustc --explain E0382`.
error: could not compile `letters` (bin "letters") due to 1 previous error
```
