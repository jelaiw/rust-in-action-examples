```sh
$ cargo add regex@1
    Updating crates.io index
      Adding regex v1 to dependencies
             Features as of v1.0.0:
             + use_std
             - pattern
             - unstable
    Updating crates.io index
$ cargo build
  Downloaded aho-corasick v1.1.3
  Downloaded regex v1.10.4
  Downloaded regex-syntax v0.8.3
  Downloaded regex-automata v0.4.6
  Downloaded memchr v2.7.2
  Downloaded 5 crates (1.5 MB) in 0.17s
   Compiling memchr v2.7.2
   Compiling regex-syntax v0.8.3
   Compiling aho-corasick v1.1.3
   Compiling regex-automata v0.4.6
   Compiling regex v1.10.4
   Compiling grep-lite v0.1.0 (/workspaces/rust-in-action-examples/grep-lite)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.89s
$
```

```sh
$ cargo doc
 Documenting memchr v2.7.2
 Documenting regex-syntax v0.8.3
 Documenting aho-corasick v1.1.3
 Documenting regex-automata v0.4.6
 Documenting regex v1.10.4
 Documenting grep-lite v0.1.0 (/workspaces/rust-in-action-examples/ch2/regex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.60s
   Generated /workspaces/rust-in-action-examples/ch2/regex/target/doc/grep_lite/index.html
$ tree -d -L 1 target/doc/
target/doc/
├── aho_corasick
├── grep_lite
├── memchr
├── regex
├── regex_automata
├── regex_syntax
├── src
├── static.files
└── trait.impl

9 directories
$
```
