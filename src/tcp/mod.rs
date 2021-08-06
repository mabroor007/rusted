use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn run(port: i32) {
    // unwrap function makes sure to stop the program if error happens
    let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    for stream in tcp_listener.incoming() {
        // this is a connection stream
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("Connection created");
    }
}

fn handle_connection(mut stream: TcpStream) {
    // this creates some space in stack where we will store the data comming from the stream
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
}
