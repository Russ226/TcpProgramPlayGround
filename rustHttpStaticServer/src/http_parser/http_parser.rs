use std::collections::HashMap;
use std::io::{BufReader, Error, Read};
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
    headers: HashMap<&'a str , &'a str>,
    body: Vec<u8>
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

pub fn parse_headers(line: &str) -> HashMap<&str , &str> {
    let split_line: Vec<_> = line.split("\r\n").collect();
    let mut headers:HashMap<&str , &str> = HashMap::new();

    for line in split_line.into_iter(){
        let header_parts: Vec<_> = line.splitn(2,":").collect();
        
        if header_parts.len() == 2 {
            headers.insert(header_parts[0].trim(), header_parts[1].trim());
        }
    }
    
    return headers;
}

pub fn parse_http_request<'a>(request: BufReader<u8>) -> Result<HttpRequest<'a>, Error>{
    let step_size = 16;
    let mut raw_string_http:String  = String::from("");
    let mut first_line: String = String::from("");
    let mut rest_of_headers: String = String::from("");
    let body: Vec<u8>;
    let mut new_line_carriage_counter = 0;
    loop {
        let mut buffer: String = String::from("");
        let cur_read = request.buffer().take(step_size).read_to_string(&mut buffer);
        if new_line_carriage_counter == 0 {
            first_line.push_str(&buffer);
        }
        if raw_string_http.ends_with("\r\n") {
            new_line_carriage_counter += 1;

            if new_line_carriage_counter == 1{
                parse_first_line(&first_line);
            } 
       }

       if raw_string_http.ends_with("\r\n\r\n"){
            break;
       }
    }




    return Ok(HttpRequest { method: HttpMethod::CONNECT, version: "()", route: "()", headers: HashMap::new(), body: Vec::new() });
}

