```sh
$ cargo r
   Compiling race v0.1.0 (/workspaces/rust-in-action-examples/ch1/race)
error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
 --> src/main.rs:5:19
  |
5 |     thread::spawn(|| { data = 500; });
  |                   ^^   ---- `data` is borrowed here
  |                   |
  |                   may outlive borrowed value `data`
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:5:5
  |
5 |     thread::spawn(|| { data = 500; });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
5 |     thread::spawn(move || { data = 500; });
  |                   ++++

error[E0499]: cannot borrow `data` as mutable more than once at a time
 --> src/main.rs:6:19
  |
5 |     thread::spawn(|| { data = 500; });
  |     ---------------------------------
  |     |             |    |
  |     |             |    first borrow occurs due to use of `data` in closure
  |     |             first mutable borrow occurs here
  |     argument requires that `data` is borrowed for `'static`
6 |     thread::spawn(|| { data = 1000; });
  |                   ^^   ---- second borrow occurs due to use of `data` in closure
  |                   |
  |                   second mutable borrow occurs here

error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
 --> src/main.rs:6:19
  |
6 |     thread::spawn(|| { data = 1000; });
  |                   ^^   ---- `data` is borrowed here
  |                   |
  |                   may outlive borrowed value `data`
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:6:5
  |
6 |     thread::spawn(|| { data = 1000; });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
6 |     thread::spawn(move || { data = 1000; });
  |                   ++++

error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
 --> src/main.rs:7:20
  |
5 |     thread::spawn(|| { data = 500; });
  |     ---------------------------------
  |     |             |    |
  |     |             |    first borrow occurs due to use of `data` in closure
  |     |             mutable borrow occurs here
  |     argument requires that `data` is borrowed for `'static`
6 |     thread::spawn(|| { data = 1000; });
7 |     println!("{}", data);
  |                    ^^^^ immutable borrow occurs here
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0373, E0499, E0502.
For more information about an error, try `rustc --explain E0373`.
error: could not compile `race` (bin "race") due to 4 previous errors
```
