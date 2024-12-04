```sh
$ rustc password.rs 
error[E0308]: mismatched types
 --> password.rs:7:31
  |
7 |     let is_strong = is_strong(pw);
  |                     --------- ^^- help: try using a conversion method: `.to_string()`
  |                     |         |
  |                     |         expected `String`, found `&str`
  |                     arguments to this function are incorrect
  |
note: function defined here
 --> password.rs:1:4
  |
1 | fn is_strong(password: String) -> bool {
  |    ^^^^^^^^^ ----------------

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
$
```
