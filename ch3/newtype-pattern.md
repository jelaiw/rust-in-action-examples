```sh
$ rustc main.rs 
error[E0308]: mismatched types
  --> main.rs:10:13
   |
10 |     connect(ordinary_string);
   |     ------- ^^^^^^^^^^^^^^^ expected `Hostname`, found `String`
   |     |
   |     arguments to this function are incorrect
   |
note: function defined here
  --> main.rs:3:4
   |
3  | fn connect(host: Hostname) {
   |    ^^^^^^^ --------------
help: try wrapping the expression in `Hostname`
   |
10 |     connect(Hostname(ordinary_string));
   |             +++++++++               +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```
