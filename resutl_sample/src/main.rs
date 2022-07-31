use std::fs::File;
use std::io::ErrorKind;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
enum Error {
    #[error("io error,{0}")]
    IoError(#[from] std::io::Error),
}

fn create_file_if_not_exists() -> Result<File, Error> {
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(error) => return Err(Error::IoError(error)),
        },
        Err(error) => return Err(Error::IoError(error)),
    };

    Ok(file)
}
fn main() {
    match create_file_if_not_exists() {
        Ok(file) => println!("{:?}", file),
        Err(error) => println!("error,{}", error.to_string()),
    }
}
