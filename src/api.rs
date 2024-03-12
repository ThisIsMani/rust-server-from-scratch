use std::{env, fs};

use crate::{request, response, types};

pub fn home(_: request::Request) -> types::ServerResult<response::Response> {
    Ok(response::Response::StatusOk)
}

pub fn echo(req: request::Request) -> types::ServerResult<response::Response> {
    let path_str = req.get_path_str()?;
    let echo_string = path_str
        .strip_prefix("/echo/")
        .ok_or(types::ServerErrors::InternalServerError)?;
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

pub fn files(req: request::Request) -> types::ServerResult<response::Response> {
    let args = env::args().collect::<Vec<String>>();
    let directory = args.get(2).ok_or(types::ServerErrors::BadRequest)?;

    let path_str = req.get_path_str()?;
    let file_name = path_str
        .strip_prefix("/files/")
        .ok_or(types::ServerErrors::InternalServerError)?;

    let file_path = format!("{}/{}", directory, file_name);
    let contents =
        fs::read_to_string(file_path).map_err(|_| types::ServerErrors::ObjectNotFound)?;

    Ok(response::Response::OctetStream(contents))
}
