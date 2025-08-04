use std::collections::HashMap;

#[path = "./http_content_type.rs"]
pub mod http_content_type;


pub struct HttpResponse{
    pub version: String,
    pub status_code: u16,
    pub status_name: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

pub fn get_status_name(status_code: u16) -> Option<String> {
    return match status_code {
        200 => Some("OK".to_string()),
        201 => Some("CREATED".to_string()),
        202 => Some("ACCEPTED".to_string()),
        204 => Some("NO CONTENT".to_string()),
        400 => Some("BAD REQUEST".to_string()),
        401 => Some("UNAUTHORIZED".to_string()),
        403 => Some("FORBIDDEN".to_string()),
        404 => Some("NOT FOUND".to_string()),
        405 => Some("METHOD NOT ALLOWED".to_string()),
        500 => Some("INTERANL SERVER ERROR".to_string()),
        _ => None
    }
}

pub fn create_http_response(status_code: u16, headers: HashMap<String, String>, body:Vec<u8>) -> HttpResponse {
    let status_code_name = match get_status_name(status_code){
        Some(s) =>  s,
        None => "".to_string()
    };

    if body.len() > 0 {
        return HttpResponse{version: "HTTP/1.1".to_owned(), status_code: status_code, status_name: status_code_name, body: body, headers: headers };
    }

    return HttpResponse{version: "HTTP/1.1".to_owned(), status_code: status_code, status_name: status_code_name, body: Vec::new(), headers: headers};
} 

pub fn http_response_to_u8(mut rep: HttpResponse) -> Vec<u8> {
    let first_line = format!("{} {} {}\r\n", rep.version, rep.status_code, rep.status_name);

    let headers = rep.headers.iter().map(|(k, v)| format!("{}:{}\r\n", k, v)).collect::<Vec<_>>().join("");

    let mut ret_item: Vec<u8> = Vec::new();

    ret_item.append(&mut first_line.as_bytes().to_vec());
    ret_item.append(&mut headers.as_bytes().to_vec());
    ret_item.append(&mut rep.body);

    return ret_item;

}

pub fn create_basic_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();

    headers.insert("Connection".to_string(), "close".to_string());
    headers.insert("Cache-Control".to_string(), "no-cache".to_string());

    return headers;
}