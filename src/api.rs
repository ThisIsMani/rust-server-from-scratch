use std::{env, fs};

use crate::{request, response, types};

pub fn home(_: request::Request) -> types::ServerResult<response::Response> {
    Ok(response::Response::StatusOk)
}

pub fn echo(req: request::Request) -> types::ServerResult<response::Response> {
    let echo_string = req.get_extra_path();
    Ok(response::Response::Text(echo_string.to_owned()))
}

pub fn user_agent(req: request::Request) -> types::ServerResult<response::Response> {
    let headers = req.get_headers();
    headers
        .into_iter()
        .find(|(name, _)| name == &"User-Agent")
        .map(|(_, value)| response::Response::Text(value.to_owned()))
        .ok_or(types::ServerErrors::BadRequest)
}

pub fn file_get(req: request::Request) -> types::ServerResult<response::Response> {
    let args = env::args().collect::<Vec<String>>();
    let directory = args.get(2).ok_or(types::ServerErrors::BadRequest)?;

    let file_name = req.get_extra_path();

    let file_path = format!("{}/{}", directory, file_name);
    let contents =
        fs::read_to_string(file_path).map_err(|_| types::ServerErrors::ObjectNotFound)?;

    Ok(response::Response::OctetStream(contents))
}

pub fn file_post(req: request::Request) -> types::ServerResult<response::Response> {
    let args = env::args().collect::<Vec<String>>();
    let directory = args.get(2).ok_or(types::ServerErrors::BadRequest)?;

    let file_name = req.get_extra_path();

    let file_path = format!("{}/{}", directory, file_name);
    fs::write(file_path, req.get_body()).map_err(|_| types::ServerErrors::InternalServerError)?;

    Ok(response::Response::Created)
}
