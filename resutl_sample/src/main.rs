use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn create_file_if_not_exists() -> Result<File, io::Error> {
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(error) => return Err(error),
        },
        Err(error) => return Err(error),
    };

    Ok(file)
}
fn main() {
    match create_file_if_not_exists() {
        Ok(file) => println!("{:?}", file),
        Err(error) => panic!("error,{}", error.to_string()),
    }
}
