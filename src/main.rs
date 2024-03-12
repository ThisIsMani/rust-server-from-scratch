use std::{io::Write, net::TcpListener, net::TcpStream, thread};

use once_cell::sync::Lazy;

mod api;
mod request;
mod response;
mod routes;
mod types;

fn handle_stream(stream: &mut TcpStream) -> types::ServerResult<response::Response> {
    let data = request::Request::new_from_stream(stream)?;
    route_request(data)
}

static ROUTES: Lazy<routes::Routes> = Lazy::new(|| routes::Routes::init());

fn route_request(req: request::Request) -> types::ServerResult<response::Response> {
    let path = req.get_path_str();
    let method = req.get_method();
    ROUTES
        .get_api_function(path, method)
        .ok_or(types::ServerErrors::UrlNotFound)
        .and_then(|api_function| api_function(req))
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let response = handle_stream(&mut stream);
                    match response {
                        Ok(response) => stream.write(response.get_string().as_bytes()),
                        Err(e) => {
                            stream.write(response::Response::from_error(e).get_string().as_bytes())
                        }
                    }
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
