use crate::types;

pub enum Response {
    StatusOk,
    Text(String),
    NotFound,
    BadRequest,
    ServerError,
    OctetStream(String),
}

impl Response {
    pub fn get_string(&self) -> String {
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
            Response::OctetStream(data) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n",
                data.len(),
                data
            ),
        }
    }

    pub fn from_error(err: types::ServerErrors) -> Self {
        match err {
            types::ServerErrors::InternalServerError => Response::ServerError,
            types::ServerErrors::PathNotFound => Response::NotFound,
            types::ServerErrors::BadRequest => Response::BadRequest,
            types::ServerErrors::ObjectNotFound => Response::NotFound,
        }
    }
}
