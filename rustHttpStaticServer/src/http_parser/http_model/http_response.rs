pub struct HttpResponse{
    pub(crate) version: String,
    pub(crate) status_code: u16,
    pub(crate) status_name: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>
}

pub fn get_status_name(status_code: u16) -> Option<String> {
    return match status_code {
        200 => "OK",
        201 => "CREATED",
        202 => "ACCEPTED",
        204 => "NO CONTENT",
        400 => "BAD REQUEST",
        401 => "UNAUTHORIZED",
        403 => "FORBIDDEN",
        404 => "NOT FOUND",
        405 => "METHOD NOT ALLOWED",
        500 => "INTERANL SERVER ERROR"
    }
}

pub fn create_http_response(status_code: u16, headers: HashMap<String, String>) -> Option<HttpResponse> {

} 

pub fn http_response_to_u8(rep: HttpResponse) -> Vec<u8> {
    
}