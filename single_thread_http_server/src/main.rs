use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7979").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let mut file = File::open("index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let status_line = "HTTP/1.1 200 OK";

    let response = format!(
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
