use anyhow::bail;
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn open_file(path_to_txt: &str) -> Result<String> {
    let path = Path::new(path_to_txt);
    if !path.exists() {
        bail!("file not found: {}", path_to_txt)
    }
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match open_file("foo.txt") {
        Ok(file) => println!("{}", file),
        Err(error) => panic!("Problem retrieving file contents: {:?}", error),
    };
}
