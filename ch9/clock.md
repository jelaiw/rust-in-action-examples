```sh
$ cargo run -- -h
   Compiling libc v0.2.171
   Compiling unicode-width v0.1.14
   Compiling textwrap v0.11.0
   Compiling num-traits v0.2.19
   Compiling atty v0.2.14
   Compiling strsim v0.8.0
   Compiling vec_map v0.8.2
   Compiling iana-time-zone v0.1.63
   Compiling ansi_term v0.12.1
   Compiling bitflags v1.3.2
   Compiling clap v2.34.0
   Compiling chrono v0.4.40
   Compiling clock v0.1.0 (/workspaces/rust-in-action-examples/ch9/clock)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.37s
     Running `target/debug/clock -h`
clock 0.1
Gets and (aspirationally) sets the time.

USAGE:
    clock [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --use-standard <std>     [default: rfc3339]  [possible values: rfc2822, rfc3339, timestamp]

ARGS:
    <action>       [default: get]  [possible values: get, set]
    <datetime>    When <action> is 'set', apply <datetime>. Otherwise, ignore.

Note: UNIX timestamps are parsed as whole seconds since 1st January 1970 0:00:00 UTC. For more accuracy, use another
format.
$ cargo run -- get -s rfc2822
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/clock get -s rfc2822`
Fri, 11 Apr 2025 16:43:23 +0000
$ cargo run -- set "Wed, 9 Apr 2025 22:22:22 +0000" -s rfc2822
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/clock set 'Wed, 9 Apr 2025 22:22:22 +0000' -s rfc2822`
Unable to set the time: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }
Fri, 11 Apr 2025 16:44:08 +0000
$ 
```
