use crate::{request, response};

#[derive(Debug)]
pub enum ServerErrors {
    InternalServerError,
    PathNotFound,
    BadRequest,
    ObjectNotFound,
}

pub type ServerResult<T> = Result<T, ServerErrors>;

pub type ServerFunction = fn(request::Request) -> ServerResult<response::Response>;
