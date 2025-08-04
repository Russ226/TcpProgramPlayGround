use std::io::Error;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MediaContentType {
    CSS,
    HTM,
    HTML,
    JPEG,
    JPG,
    JSON,
    PHP,
    XML,
    JS,
    UNKNOWN
}

pub fn get_media_content_type_header(content_type: MediaContentType) -> Result<String, Error> {
    return match content_type {
        MediaContentType::CSS => Ok("text/css".to_string()),
        MediaContentType::HTM | MediaContentType::HTML => Ok("text/html".to_string()),
        MediaContentType::JPEG | MediaContentType::JPG => Ok("image/jpeg".to_string()),
        MediaContentType::JSON => Ok("application/json".to_string()),
        MediaContentType::PHP => Ok("application/x-httpd-php".to_string()),
        MediaContentType::JS => Ok("text/javascript".to_string()),
        MediaContentType::XML => Ok("application/xml".to_string()),
        _ => Err(Error::other("Invalid Media type")) 
    }
}


pub fn get_media_content_type(content_type: String) -> Option<MediaContentType> {
    return match content_type {
        val if val == "text/css".to_string() => Some(MediaContentType::CSS),
        val if val =="text/html".to_string() => Some(MediaContentType::HTML),
        val if val =="image/jpeg".to_string() => Some(MediaContentType::JPEG),
        val if val =="application/json".to_string() => Some(MediaContentType::JSON),
        val if val =="application/x-httpd-php".to_string() => Some(MediaContentType::PHP),
        val if val =="application/xml".to_string() => Some(MediaContentType::XML),
        val if val =="text/javascript".to_string() => Some(MediaContentType::JS),
        _ => None
    }
}