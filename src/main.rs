use std::{io::Write, net::TcpListener, net::TcpStream, thread};

mod api;
mod request;
mod response;
mod types;

fn handle_stream(mut stream: TcpStream) {
    let data = request::Request::new_from_stream(&mut stream);
    match handle_request(data) {
        Ok(response) => stream.write(response.get_string().as_bytes()),
        Err(e) => stream.write(response::Response::from_error(e).get_string().as_bytes()),
    }
    .ok();
}

fn handle_request(req: request::Request) -> types::ServerResult<response::Response> {
    let path = req.get_path()?;
    match path {
        request::Path::Home(fun) => fun(req),
        request::Path::Echo(fun) => fun(req),
        request::Path::UserAgent(fun) => fun(req),
        request::Path::Files(fun) => fun(req),
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_stream(stream));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
