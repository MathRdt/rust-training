use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut username = String::new();

    File::open("hello.txt")?;

    Ok(())
}
