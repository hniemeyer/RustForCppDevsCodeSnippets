use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

fn open_file() -> Result<String> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match open_file() {
        Ok(file) => println!("{}", file),
        Err(error) => panic!("Problem retrieving file contents: {:?}", error),
    };
}
