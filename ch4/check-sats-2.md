```sh
$ cargo run
   Compiling check-sats-2 v0.1.0 (/workspaces/rust-in-action-examples/ch4/check-sats-2)
error[E0382]: use of moved value: `sat_a`
  --> src/main.rs:27:33
   |
17 |     let sat_a = CubeSat { id: 0 };
   |         ----- move occurs because `sat_a` has type `CubeSat`, which does not implement the `Copy` trait
...
21 |     let a_status = check_status(sat_a);
   |                                 ----- value moved here
...
27 |     let a_status = check_status(sat_a);
   |                                 ^^^^^ value used here after move
   |
note: consider changing this parameter type in function `check_status` to borrow instead if owning the value isn't necessary
  --> src/main.rs:12:25
   |
12 | fn check_status(sat_id: CubeSat) -> StatusMessage {
   |    ------------         ^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `CubeSat` implemented `Clone`, you could clone the value
  --> src/main.rs:3:1
   |
3  | struct CubeSat {
   | ^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
21 |     let a_status = check_status(sat_a);
   |                                 ----- you could clone this value

error[E0382]: use of moved value: `sat_b`
  --> src/main.rs:28:33
   |
18 |     let sat_b = CubeSat { id: 1 };
   |         ----- move occurs because `sat_b` has type `CubeSat`, which does not implement the `Copy` trait
...
22 |     let b_status = check_status(sat_b);
   |                                 ----- value moved here
...
28 |     let b_status = check_status(sat_b);
   |                                 ^^^^^ value used here after move
   |
note: consider changing this parameter type in function `check_status` to borrow instead if owning the value isn't necessary
  --> src/main.rs:12:25
   |
12 | fn check_status(sat_id: CubeSat) -> StatusMessage {
   |    ------------         ^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `CubeSat` implemented `Clone`, you could clone the value
  --> src/main.rs:3:1
   |
3  | struct CubeSat {
   | ^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
22 |     let b_status = check_status(sat_b);
   |                                 ----- you could clone this value

error[E0382]: use of moved value: `sat_c`
  --> src/main.rs:29:33
   |
19 |     let sat_c = CubeSat { id: 2 };
   |         ----- move occurs because `sat_c` has type `CubeSat`, which does not implement the `Copy` trait
...
23 |     let c_status = check_status(sat_c);
   |                                 ----- value moved here
...
29 |     let c_status = check_status(sat_c);
   |                                 ^^^^^ value used here after move
   |
note: consider changing this parameter type in function `check_status` to borrow instead if owning the value isn't necessary
  --> src/main.rs:12:25
   |
12 | fn check_status(sat_id: CubeSat) -> StatusMessage {
   |    ------------         ^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `CubeSat` implemented `Clone`, you could clone the value
  --> src/main.rs:3:1
   |
3  | struct CubeSat {
   | ^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
23 |     let c_status = check_status(sat_c);
   |                                 ----- you could clone this value

For more information about this error, try `rustc --explain E0382`.
error: could not compile `check-sats-2` (bin "check-sats-2") due to 3 previous errors
```
