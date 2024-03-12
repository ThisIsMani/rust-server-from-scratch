use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::net::TcpStream;

use crate::types;
use crate::types::ServerErrors;

#[derive(Debug)]
pub struct Request {
    headers: Headers,
    body: String,
}

#[derive(Debug)]
pub struct Headers {
    method: Method,
    path: String,
    headers: HashMap<String, String>,
}

impl Headers {
    pub fn new(data: String) -> types::ServerResult<Self> {
        let (method, remaining_headers) = data
            .split_once(' ')
            .ok_or(types::ServerErrors::BadRequest)?;

        let method = Method::try_from(method)?;

        let (path, remaining_headers) = remaining_headers
            .split_once(' ')
            .ok_or(types::ServerErrors::BadRequest)?;

        let headers = remaining_headers
            .lines()
            .into_iter()
            .filter_map(|header| header.split_once(": "))
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .collect();

        Ok(Self {
            method,
            path: path.to_owned(),
            headers,
        })
    }

    fn get_path(&self) -> String {
        self.path.to_owned()
    }

    fn get_method(&self) -> Method {
        self.method
    }

    fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}

impl Request {
    pub fn new_from_stream(stream: &mut TcpStream) -> types::ServerResult<Self> {
        let mut res = String::new();
        let mut reader = BufReader::new(stream);
        while let Ok(n) = reader.read_line(&mut res) {
            if n == 0 || res.ends_with("\r\n\r\n") {
                break;
            }
        }
        let headers = Headers::new(res)?;
        let content_length: usize = headers
            .get_headers()
            .get("Content-Length")
            .map(|x| x.parse())
            .transpose()
            .map_err(|_| types::ServerErrors::BadRequest)?
            .unwrap_or(0);

        let mut body = vec![0; content_length];
        reader
            .read_exact(&mut body)
            .map_err(|_| types::ServerErrors::InternalServerError)?;

        Ok(Self {
            headers,
            body: String::from_utf8(body).map_err(|_| types::ServerErrors::InternalServerError)?,
        })
    }

    pub fn get_path_str(&self) -> String {
        self.headers.get_path()
    }

    pub fn get_extra_path(&self) -> String {
        self.get_path_str()
            .strip_prefix('/')
            .and_then(|path| path.split_once('/').map(|(_, extra)| extra))
            .unwrap_or("")
            .to_owned()
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.get_headers()
    }

    pub fn get_method(&self) -> Method {
        self.headers.get_method()
    }

    pub fn get_body(&self) -> String {
        self.body.to_owned()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

impl TryFrom<&str> for Method {
    type Error = types::ServerErrors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => Err(ServerErrors::BadRequest),
        }
    }
}
