```sh
$ rustc multierror.rs 
error[E0277]: `?` couldn't convert the error to `std::io::Error`
 --> multierror.rs:7:47
  |
4 | fn main() -> Result<(), std::io::Error> {
  |              -------------------------- expected `std::io::Error` because of this
...
7 |     let _localhost = "::1".parse::<Ipv6Addr>()?;
  |                            -------------------^ the trait `From<AddrParseError>` is not implemented for `std::io::Error`, which is required by `Result<(), std::io::Error>: FromResidual<Result<Infallible, AddrParseError>>`
  |                            |
  |                            this can't be annotated with `?` because it has type `Result<_, AddrParseError>`
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
  = help: the following other types implement trait `From<T>`:
            `std::io::Error` implements `From<ErrorKind>`
            `std::io::Error` implements `From<IntoInnerError<W>>`
            `std::io::Error` implements `From<NulError>`
            `std::io::Error` implements `From<TryReserveError>`
  = note: required for `Result<(), std::io::Error>` to implement `FromResidual<Result<Infallible, AddrParseError>>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```
