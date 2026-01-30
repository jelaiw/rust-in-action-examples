```sh
$ rustc ok.rs 
$ ./ok
ok
$ mkdir ok
$ mv ok.rs ok
$ cd ok
$ cargo init
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cargo run
   Compiling ok v0.1.0 (/workspaces/rust-in-action-examples/ch1/ok)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/ok`
ok
$ cargo clean
     Removed 21 files, 7.5MiB total
$ cargo run -v
   Compiling ok v0.1.0 (/workspaces/rust-in-action-examples/ch1/ok)
     Running `/usr/local/rustup/toolchains/1.93.0-x86_64-unknown-linux-gnu/bin/rustc --crate-name ok --edition=2024 ok.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=176 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=e1019fca702a64c1 -C extra-filename=-f64048773a800dd9 --out-dir /workspaces/rust-in-action-examples/ch1/ok/target/debug/deps -C incremental=/workspaces/rust-in-action-examples/ch1/ok/target/debug/incremental -L dependency=/workspaces/rust-in-action-examples/ch1/ok/target/debug/deps`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/ok`
ok
$
```
