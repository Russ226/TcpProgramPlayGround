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

pub fn get_media_content_type_header(content_type: MediaContentType) -> String{
    return match content_type {
        MediaContentType::CSS => "text/css".to_string(),
        MediaContentType::HTM | MediaContentType::HTML => "text/html".to_string(),
        MediaContentType::JPEG | MediaContentType::JPG => "image/jpeg".to_string(),
        MediaContentType::JSON => "application/json".to_string(),
        MediaContentType::PHP => "application/x-httpd-php".to_string(),
        MediaContentType::JS => "text/javascript".to_string(),
        MediaContentType::XML => "application/xml".to_string(),
        _ => "application/octet-stream".to_string(),
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
        val if val =="application/octet-stream".to_string() => Some(MediaContentType::UNKNOWN),
        _ => None
    }
}

 pub fn file_extesion_to_media_content(ext: String) -> MediaContentType{
    return match ext {
        val if val == "js".to_string() => MediaContentType::JS,
        val if val == "css".to_string() => MediaContentType::CSS,
        val if val == "html".to_string() => MediaContentType::HTML,
        val if val == "htm".to_string() => MediaContentType::HTM,
        val if val == "json".to_string() => MediaContentType::JSON,
        val if val == "xml".to_string() => MediaContentType::XML,
        val if val == "php".to_string() => MediaContentType::PHP,
        _ => MediaContentType::UNKNOWN
        
    } 
 }