// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
    net::TcpStream,
};

fn get_data_from_streams(stream: &mut TcpStream) -> String {
    let mut res = [0; 20];
    let _ = stream.read_exact(&mut res);
    String::from_utf8(res.to_vec()).unwrap()
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let data = get_data_from_streams(&mut stream);
                let x = data.split(' ').nth(1);
                match x {
                    Some("/") => stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
                    _ => stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap(),
                };
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
