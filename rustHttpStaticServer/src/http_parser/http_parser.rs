use std::collections::HashMap;
use std::io::{Error, Read};
use std::result::Result;

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

#[derive(Debug)]
pub struct HttpRequest {
    pub(crate) method: HttpMethod,
    pub(crate) version: String,
    pub(crate) route: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>,
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

pub fn parse_first_line<'a>(line: &String) -> Result<(HttpMethod, String, String), Error> {
    let method: HttpMethod;
    let version: String;
    let route: String;

    let split_line: Vec<_> = line.trim().split(" ").collect();

    if split_line.len() != 3 {
        Error::other("Invalid Http Request");
    }

    method = get_http_method(split_line[0]).unwrap();
    version = String::from(split_line[1]);
    route = String::from(split_line[2]);

    return Ok((method, version, route));
}

pub fn parse_headers(line: String) -> HashMap<String, String> {
    let split_line: Vec<_> = line.split("\r\n").collect();
    let mut headers: HashMap<String, String> = HashMap::new();

    for line in split_line.into_iter() {
        let header_parts: Vec<_> = line.splitn(2, ":").collect();

        if header_parts.len() == 2 {
            let key: String = String::from(header_parts[0].trim());
            let value: String = String::from(header_parts[1].trim());

            headers.insert(key, value);
        }
    }

    return headers;
}

pub fn parse_http_request(mut request: Vec<u8>) -> Result<HttpRequest, Error> {
    let step_size = 1;
    let mut first_line: String = String::new();
    let mut rest_of_headers: String = String::new();
    let mut body: Vec<u8> = Vec::new();
    let mut parsed_first_line = false;

    let mut http_method: HttpMethod = HttpMethod::INVALID;
    let mut version: String = String::new();
    let mut route: String = String::new();
    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let mut buffer: String = String::from("");
        match request.take(step_size).read_to_string(&mut buffer) {
            Ok(r) => request.drain(0..r),
            Err(e) => {
                println!("Failed to parse http request, with error {}", e);

                return Err(e);
            }
        };

        match parsed_first_line {
            false => {
                first_line.push_str(&buffer);

                if first_line.ends_with("\r\n") && !parsed_first_line {
                    parsed_first_line = true;

                    match parse_first_line(&first_line) {
                        Ok(line) => {
                            http_method = line.0;
                            route = String::from(line.1);
                            version = String::from(line.2);
                        }
                        Err(e) => {
                            println!("failed to parse first line: {}", e);

                            return Err(e);
                        }
                    }
                }
            }
            true => {
                rest_of_headers.push_str(&buffer);
                if (rest_of_headers.ends_with("\r\n\r\n") || request.len() == 0) && parsed_first_line {
                    headers = parse_headers(rest_of_headers);

                    if headers.contains_key("Content-Length") && request.len() > 0 {
                        match headers["Content-Length"].parse::<u64>() {
                            Ok(size) => match request.take(size).read_to_end(&mut body) {
                                Ok(size) => println!("Body is size {}", size),
                                Err(e) => println!("Unable to read body, {}", e),
                            },
                            Err(e) => println!("unable to parse content-length {}", e),
                        }
                    }
                    break;
                }
            }
        }
    }

    let ret_item: HttpRequest = HttpRequest {
        method: http_method,
        version: version,
        route: route,
        headers: headers,
        body: body,
    };

    return Ok(ret_item);
}
