use std::path::{Path};
use std::fs::File;

pub fn file_handler(root_path: &Path, route: String) -> Option<File> {
    let combined_path = root_path.join(route.replacen('/', "", 1));

    
    if combined_path.is_dir() {
        let file = File::open(combined_path.join("index.html"));
        return match file {
            Ok(f) => Some(f),
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
            Ok(f) => Some(f),
            Err(e) => {
                println!("Error opening index.html at {:?} with error {}", combined_path.as_os_str(), e);
                return None;
            }
        }
    }

    return None;

    
}