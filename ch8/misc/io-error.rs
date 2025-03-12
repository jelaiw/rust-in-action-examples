use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let _f:File = File::open("foo.txt")?;

    Ok(())
}
