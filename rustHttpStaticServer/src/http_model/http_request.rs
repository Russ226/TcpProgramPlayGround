use std::collections::HashMap;

#[path = "./http_method.rs"]
mod http_method;

#[derive(Debug)]
pub struct HttpRequest {
    pub(crate) method: http_method::HttpMethod,
    pub(crate) version: String,
    pub(crate) route: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>,
}