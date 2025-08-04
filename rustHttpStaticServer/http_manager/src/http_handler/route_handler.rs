use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path};
use std::fs::File;

use crate::http_model;

pub fn file_handler(root_path: &Path, route: String) -> Option<(File, String)> {
    let combined_path = root_path.join(route.replacen('/', "", 1));

    
    if combined_path.is_dir() {
        let file = File::open(combined_path.join("index.html"));
        return match file {
            Ok(f) => Some((f, "html".to_string())),
            Err(e) => {
                println!("Error opening index.html at {:?} with error {}", combined_path.as_os_str(), e);
                return None;
            }
        }
    }

    if combined_path.is_file() {
        let combined_path = root_path.join(route.replacen('/', "", 1));
        let file = File::open(&combined_path);
        return match file {
            Ok(f) => {
                match combined_path.extension().and_then(OsStr::to_str) {
                    Some(s) =>{
                        return Some((f, s.to_string()))
                    },
                    None => None   
                }
            }  
                
            Err(e) => {
                println!("Error opening index.html at {:?} with error {}", combined_path.as_os_str(), e);
                return None;
            }
        }
    }

    return None;    
}

pub fn http_method_handler(root_path: &Path, request: http_model::http_request::HttpRequest)-> http_model::http_response::HttpResponse{
    match request {
        val if val.method == http_model::http_method::HttpMethod::GET => {
            match file_handler(root_path, val.route.clone()){
                Some(mut f) => {
                    let mut body: Vec<u8> = Vec::new(); 
                    let status_code = 200;
                    let mut headers = http_model::http_response::create_basic_headers();
                    
                    match f.0.read_to_end(&mut body){
                        Ok(_) => {},
                        Err(e) => {
                            println!("error reading file for {} with message {}", val.route, e);
                            let status_code = 500;
                            let mut headers  = http_model::http_response::create_basic_headers();
                            return http_model::http_response::create_http_response(status_code, headers, body);
                        }
                    };

                    return http_model::http_response::create_http_response(status_code, headers, body);
                }

                None => {
                    let status_code = 404;
                    let mut headers  = http_model::http_response::create_basic_headers();
                    return http_model::http_response::create_http_response(status_code, headers, Vec::new());
                }
            }
        },
        _ => {
            let status_code = 405;
            let mut headers  = http_model::http_response::create_basic_headers();
            headers.insert("Allow".to_string(), "GET".to_string());

            return http_model::http_response::create_http_response(status_code, headers, Vec::new());
        }

    }
}