use std::io::Error;

#[derive(PartialEq, Eq, Debug, Clone)]
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
    INVALID,
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
        "CONNECT" => Ok(HttpMethod::CONNECT),
        _ => Err(Error::other("Invalid Http Method")),
    };
}