use std::collections::HashMap;
use std::io::Error;
use std::result::Result;

#[derive(PartialEq, Eq, Debug)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

pub struct HttpRequest<'a> {
    method: HttpMethod,
    version: &'a str,
    route: &'a str,
    headers: HashMap<&'a str, &'a str>,
}

pub fn get_http_method(m: &str) -> Result<HttpMethod, Error> {
    return match m {
        "GET" => Ok(HttpMethod::GET),
        "HEAD" => Ok(HttpMethod::HEAD),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "DELETE" => Ok(HttpMethod::DELETE),
        "OPTIONS" => Ok(HttpMethod::OPTIONS),
        "TRACE" => Ok(HttpMethod::TRACE),
        "PATCH" => Ok(HttpMethod::PATCH),
        _ => Err(Error::other("Invalid Http Method")),
    };
}

pub fn parse_first_line(line: &str) -> Result<(HttpMethod, &str, &str), Error> {
    let method: HttpMethod;
    let version: &str;
    let route: &str;

    let split_line: Vec<_> = line.trim().split(" ").collect();

    if split_line.len() != 3 {
        Error::other("Invalid Http Request");
    }

    method = get_http_method(split_line[0]).unwrap();
    version = split_line[1];
    route = split_line[2];

    return Ok((method, version, route));
}

