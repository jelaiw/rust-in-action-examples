```sh
$ cargo r
   Compiling fruit v0.1.0 (/workspaces/rust-in-action-examples/ch1/fruit)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
     Running `target/debug/fruit`
thread 'main' panicked at src/main.rs:4:32:
index out of bounds: the len is 3 but the index is 4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
