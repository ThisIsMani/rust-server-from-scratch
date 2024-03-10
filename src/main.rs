// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    net::TcpStream,
};

#[derive(Debug)]
enum ServerErrors {
    InternalServerError,
    PathNotFound,
    BadRequest,
}

type ServerResult<T> = Result<T, ServerErrors>;

type ServerFunction = fn(Request) -> ServerResult<Response>;

enum Path {
    Home(ServerFunction),
    Echo(ServerFunction),
    UserAgent(ServerFunction),
}

#[derive(Debug)]
struct Request {
    inner: String,
}

enum Response {
    StatusOk,
    Text(String),
    NotFound,
    BadRequest,
    ServerError,
}

impl Response {
    fn get_string(&self) -> String {
        match self {
            Response::StatusOk => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
            Response::Text(text) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
                text.len(),
                text
            ),
            Response::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
            Response::BadRequest => "HTTP/1.1 400 Bad Request\r\n\r\n".to_string(),
            Response::ServerError => "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string(),
        }
    }

    fn from_error(err: ServerErrors) -> Self {
        match err {
            ServerErrors::InternalServerError => Response::ServerError,
            ServerErrors::PathNotFound => Response::NotFound,
            ServerErrors::BadRequest => Response::BadRequest,
        }
    }
}

impl Request {
    fn new(data: String) -> Self {
        Self { inner: data }
    }

    fn get_path(&self) -> Result<Path, ServerErrors> {
        let path = self.get_path_str()?;

        if path.starts_with("/echo") {
            return Ok(Path::Echo(echo));
        } else if path == "/" {
            return Ok(Path::Home(home));
        } else if path == "/user-agent" {
            return Ok(Path::UserAgent(user_agent));
        } else {
            return Err(ServerErrors::PathNotFound);
        }
    }

    fn get_path_str(&self) -> Result<&str, ServerErrors> {
        let url_path = self.inner.split(' ').nth(1);
        url_path.ok_or(ServerErrors::PathNotFound)
    }

    fn get_headers(&self) -> HashMap<&str, &str> {
        self.inner
            .split('\n')
            .into_iter()
            .filter_map(|data| data.split_once(": "))
            .collect()
    }
}

fn get_request_from_stream(stream: &mut TcpStream) -> Request {
    let mut res = String::new();
    let mut reader = BufReader::new(stream);
    while let Ok(n) = reader.read_line(&mut res) {
        if n == 0 || res.ends_with("\r\n\r\n") {
            break;
        }
    }
    Request::new(res)
}

fn handle_stream(mut stream: TcpStream) {
    let data = get_request_from_stream(&mut stream);
    match handle_request(data) {
        Ok(response) => stream.write(response.get_string().as_bytes()),
        Err(e) => stream.write(Response::from_error(e).get_string().as_bytes()),
    }
    .ok();
}

fn handle_request(req: Request) -> Result<Response, ServerErrors> {
    let path = req.get_path()?;
    match path {
        Path::Home(fun) => fun(req),
        Path::Echo(fun) => fun(req),
        Path::UserAgent(fun) => fun(req),
    }
}

fn home(_: Request) -> ServerResult<Response> {
    Ok(Response::StatusOk)
}

fn echo(req: Request) -> ServerResult<Response> {
    let path_str = req.get_path_str()?;
    let echo_string = path_str
        .strip_prefix("/echo/")
        .ok_or(ServerErrors::InternalServerError)?;
    Ok(Response::Text(echo_string.to_owned()))
}

fn user_agent(req: Request) -> ServerResult<Response> {
    let headers = req.get_headers();
    println!("{:#?}", headers);
    headers
        .into_iter()
        .find(|(name, _)| name == &"User-Agent")
        .map(|(_, value)| Response::Text(value.to_owned()))
        .ok_or(ServerErrors::BadRequest)
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(e) => println!("Error: {}", e),
        }
    }
}
