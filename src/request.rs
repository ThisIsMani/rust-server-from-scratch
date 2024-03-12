use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

use crate::api;
use crate::types;

#[derive(Debug)]
pub struct Request {
    inner: String,
}

impl Request {
    pub fn new_from_stream(stream: &mut TcpStream) -> Self {
        let mut res = String::new();
        let mut reader = BufReader::new(stream);
        while let Ok(n) = reader.read_line(&mut res) {
            if n == 0 || res.ends_with("\r\n\r\n") {
                break;
            }
        }
        Self { inner: res }
    }

    pub fn get_path(&self) -> types::ServerResult<Path> {
        let path = self.get_path_str()?;

        if path.starts_with("/echo") {
            return Ok(Path::Echo(api::echo));
        } else if path == "/" {
            return Ok(Path::Home(api::home));
        } else if path == "/user-agent" {
            return Ok(Path::UserAgent(api::user_agent));
        } else if path.starts_with("/files") {
            return Ok(Path::Files(api::files));
        } else {
            return Err(types::ServerErrors::PathNotFound);
        }
    }

    pub fn get_path_str(&self) -> types::ServerResult<&str> {
        let url_path = self.inner.split(' ').nth(1);
        url_path.ok_or(types::ServerErrors::PathNotFound)
    }

    pub fn get_headers(&self) -> HashMap<&str, &str> {
        self.inner
            .split('\n')
            .into_iter()
            .filter_map(|data| {
                data.strip_suffix("\r")
                    .and_then(|header| header.split_once(": "))
            })
            .collect()
    }
}

pub enum Path {
    Home(types::ServerFunction),
    Echo(types::ServerFunction),
    UserAgent(types::ServerFunction),
    Files(types::ServerFunction),
}
