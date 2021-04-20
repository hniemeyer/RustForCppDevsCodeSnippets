use std::io::Error;
use std::fs::File;

// always fails for the sake of this example
fn open_file() -> Result<File, Error> {
    File::open("does_not_exist.txt")
}

fn bubble_up() -> Result<File, Error> {
    // TODO: how can we use `?` to shorten the error handling here?
    let file = match open_file() {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    println!("Found file: {:?}", file);
    Ok(file)
}

fn panic_on_err() {
    // TODO: fail loudly if `file_content()` fails, by using .unwrap() or triggering a panic!()
}

fn main() {
    let bubble_up_err = bubble_up();
    println!("bubble_up() yielded: {:?}", bubble_up_err);
    println!("panic_on_err() yielded: {:?}", panic_on_err());
}