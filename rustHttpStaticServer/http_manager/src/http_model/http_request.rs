use std::collections::HashMap;

use crate::http_model::http_method;



#[derive(Debug)]
pub struct HttpRequest {
    pub method: http_method::HttpMethod,
    pub version: String,
    pub route: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}