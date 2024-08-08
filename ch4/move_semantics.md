```sh
$ rustc move_semantics.rs 
error[E0382]: borrow of moved value: `demo`
  --> move_semantics.rs:12:20
   |
9  |     let demo = Demo { a: 123 };
   |         ---- move occurs because `demo` has type `Demo`, which does not implement the `Copy` trait
10 |     use_value(demo);
   |               ---- value moved here
11 |
12 |     println!("{}", demo.a);
   |                    ^^^^^^ value borrowed here after move
   |
note: consider changing this parameter type in function `use_value` to borrow instead if owning the value isn't necessary
  --> move_semantics.rs:5:20
   |
5  | fn use_value(_val: Demo) {
   |    ---------       ^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `Demo` implemented `Clone`, you could clone the value
  --> move_semantics.rs:1:1
   |
1  | struct Demo {
   | ^^^^^^^^^^^ consider implementing `Clone` for this type
...
10 |     use_value(demo);
   |               ---- you could clone this value
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```
