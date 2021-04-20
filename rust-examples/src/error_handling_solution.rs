use std::io::Error;
use std::fs::File;

// always fails for the sake of this example
fn open_file() -> Result<File, Error> {
    File::open("does_not_exist.txt")
}

fn bubble_up() -> Result<File, Error> {
    let file = open_file()?;

    println!("Found file: {:?}", file);
    Ok(file)
}

fn panic_on_err() {
    let file_content = open_file().unwrap();

    println!("File contains {:?}", file_content);
}

fn main() {
    let bubble_up_err = bubble_up();
    println!("bubble_up() yielded: {:?}", bubble_up_err);
    println!("panic_on_err() yielded: {:?}", panic_on_err());
}