```sh
$ cargo run
   Compiling cereals v0.1.0 (/workspaces/rust-in-action-examples/ch1/cereals)
error[E0382]: borrow of moved value: `grains`
  --> main.rs:11:22
   |
8  |     let mut grains: Vec<Cereal> = vec![];
   |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
9  |     grains.push(Cereal::Rye);
10 |     drop(grains);
   |          ------ value moved here
11 |     println!("{:?}", grains);
   |                      ^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `cereals` (bin "cereals") due to 1 previous error
```
