use std::path::{Path};
use std::fs::File;

pub fn file_handler(route: String) -> Option<File> {
    let route_path = Path::new(&route);
    
    if route_path.is_dir() {
        let file = File::open(route_path.join("index.html"));
        return match file {
            Ok(f) => Some(f),
            Err(e) => {
                println!("Error opening index.html at {:?} with error {}", route_path.as_os_str(), e);
                return None;
            }
        }
    }

    if route_path.is_file() {
         
    }

    
}