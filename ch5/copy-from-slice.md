```sh
$ rustc copy-from-slice.rs 
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> copy-from-slice.rs:13:22
    |
13  |     println!("{:?}", mem[0x100..0x106]);
    |               ----   ^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |               |
    |               required by a bound introduced by this call
    |
    = help: the trait `Sized` is not implemented for `[u8]`
note: required by an implicit `Sized` bound in `core::fmt::rt::Argument::<'a>::new_debug`
   --> /usr/local/rustup/toolchains/1.81.0-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:117:26
    |
117 |     pub fn new_debug<'b, T: Debug>(x: &'b T) -> Argument<'_> {
    |                          ^ required by the implicit `Sized` requirement on this type parameter in `Argument::<'a>::new_debug`
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```
