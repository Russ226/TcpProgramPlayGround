#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Read, path::{PathBuf, Path}
    };

    extern crate http_manager;

    #[test]
    fn join_paths_test(){
        let mut root_path = PathBuf::from("c:/Users/Russ2");
        let path =  PathBuf::from("foo/bar.txt");

        root_path.push(path);

        assert_eq!(root_path, Path::new("c:/Users/Russ2/foo/bar.txt"));
    }
   

    #[test]
    fn test_parse_first_line_1() {
        let first_line_header = String::from("GET / HTTP/1.1\r\n");

        let result = http_manager::http_parser::http_parser::parse_first_line(&first_line_header).unwrap();

        assert_eq!(result.0, http_manager::http_model::http_method::HttpMethod::GET);
        assert_eq!(result.1, "/");
        assert_eq!(result.2, "HTTP/1.1");
    }

    #[test]
    fn test_parse_first_line_2() {
        let first_line_header = String::from("POST /test HTTP/1.1\r\n");

        let result = http_manager::http_parser::http_parser::parse_first_line(&first_line_header).unwrap();

        assert_eq!(result.0, http_manager::http_model::http_method::HttpMethod::POST);
        assert_eq!(result.1, "/test");
        assert_eq!(result.2, "HTTP/1.1");
    }
    #[test]
    fn parse_headers_1() {
        let headers = fs::read_to_string("tests\\headers1.txt")
            .expect("Should have been able to read the file");

        let result = http_manager::http_parser::http_parser::parse_headers(headers);

        assert_eq!(result["Host"], "127.0.0.1:8080");
        assert_eq!(result["Connection"], "keep-alive");
        assert_eq!(result["Sec-Fetch-User"], "?1");
        assert_eq!(result["Accept-Language"], "en-US,en;q=0.9");
    }

    #[test]
    fn parse_headers_2() {
        let headers = fs::read_to_string("tests\\headers2.txt")
            .expect("Should have been able to read the file");

        let result = http_manager::http_parser::http_parser::parse_headers(headers);

        assert_eq!(result["Host"], "127.0.0.1:8080");
        assert_eq!(result["Connection"], "keep-alive");
        assert_eq!(result["User-Agent"], "PostmanRuntime/7.26.8");
        assert_eq!(
            result["Postman-Token"],
            "1f5a5c52-d86d-495c-93e4-5aaee56390ea"
        );
    }

    #[test]
    fn parse_http_request_1() {
        let mut file = File::open("tests\\httpGetDeafultPath.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\httpGetDeafultPath.txt"),
            Err(e) => panic!("Failed to read tests\\httpGetDeafultPath.txt, {}", e),
        }

        let result = http_manager::http_parser::http_parser::parse_http_request(buffer, true).unwrap();

        assert_eq!(result.method, http_manager::http_model::http_method::HttpMethod::GET);
        assert_eq!(result.route, "/");
        assert_eq!(result.version, "HTTP/1.1");

        assert_eq!(result.headers["Host"], "127.0.0.1:8080");
        assert_eq!(result.headers["Connection"], "keep-alive");
        assert_eq!(result.headers["Sec-Fetch-User"], "?1");
        assert_eq!(result.headers["Accept-Language"], "en-US,en;q=0.9");

        assert_eq!(result.body.len(), 0);
    }

    #[test]
    fn parse_http_request_2() {
        let result_body = r#"{"user":"test","id":1,"isActive":false}"#;

        let mut file = File::open("tests\\httpHostTestControllerjson.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\httpHostTestControllerjson.txt"),
            Err(e) => panic!("Failed to read tests\\httpHostTestControllerjson.txt, {}", e),
        }

        let result =  http_manager::http_parser::http_parser::parse_http_request(buffer, true).unwrap();

        assert_eq!(result.method, http_manager::http_model::http_method::HttpMethod::POST);
        assert_eq!(result.route, "/test");
        assert_eq!(result.version, "HTTP/1.1");

        assert_eq!(result.headers["Host"], "127.0.0.1:8080");
        assert_eq!(result.headers["Connection"], "keep-alive");
        assert_eq!(result.headers["Postman-Token"], "1f5a5c52-d86d-495c-93e4-5aaee56390ea");
        assert_eq!(result.headers["Content-Length"], "61");

        assert_eq!(result.body.len(), 61);

        let str_body = match String::from_utf8(result.body){
            Ok(s) => s,
            Err(e) => panic!("{}", e)
        };
        assert_eq!(str_body.replace("\r\n", "").replace(" ", ""), result_body);


    }

    #[test]
    fn test_http_file_handler_dir(){
        let root_path = Path::new("C:/Users/russ2/Desktop/TcpPrograms");
        let route = String::from("/route1");

        match http_manager::http_handler::route_handler::file_handler(root_path, route){
            Some(mut f) => {
                let mut expect_file_result = match File::open("C:\\Users\\russ2\\Desktop\\TcpPrograms\\route1\\index.html"){
                    Ok(f) => f,
                    Err(e) => panic!("Failed to open C:\\Users\\russ2\\Desktop\\TcpPrograms\\route1\\index.html for expected result, {}", e)
                };
                let mut route_str = String::from(""); 
                f.0.read_to_string(&mut route_str).expect("failed to convert file contents to string for route");

                let mut index_html_str = String::from(""); 
                expect_file_result.read_to_string(&mut index_html_str).expect("failed to convert file contents to string for expected result");

                assert_eq!(route_str, index_html_str);
                assert_eq!(f.1, "html");

            },
            None => panic!("Failed to open to handle dir route, /route1")
        };
    }

    #[test]
    fn test_http_file_handler_file(){
        let root_path = Path::new("C:/Users/russ2/Desktop/TcpPrograms");
        let route = String::from("/test.js");

        match http_manager::http_handler::route_handler::file_handler(root_path, route){
            Some(mut f) => {
                let mut expect_file_result = match File::open("C:\\Users\\russ2\\Desktop\\TcpPrograms\\test.js"){
                    Ok(f) => f,
                    Err(e) => panic!("Failed to open C:\\Users\\russ2\\Desktop\\TcpPrograms\\test.js for expected result, {}", e)
                };
                let mut route_str = String::from(""); 
                f.0.read_to_string(&mut route_str).expect("failed to convert file contents to string for route");

                let mut index_html_str = String::from(""); 
                expect_file_result.read_to_string(&mut index_html_str).expect("failed to convert file contents to string for expected result");

                assert_eq!(route_str, index_html_str);
                assert_eq!(f.1, "js");

            },
            None => panic!("Failed to open to handle dir route, /route1")
        };
    }

    #[test]
    fn test_http_response_200_default_index(){
        let mut file = File::open("tests\\httpGetDeafultPath.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\httpGetDeafultPath.txt"),
            Err(e) => panic!("Failed to read tests\\httpGetDeafultPath.txt, {}", e),
        }

        let root_path = Path::new("C:\\Users\\russ2\\Desktop\\TcpPrograms");
        let http_request =  http_manager::http_parser::http_parser::parse_http_request(buffer, true).unwrap();

        let http_response = http_manager::http_handler::route_handler::http_method_handler(root_path, http_request);

        assert_eq!(http_response.status_code, 200);
        assert_eq!(http_response.status_name, "OK".to_string());
        assert_eq!(http_response.headers.get("content-length").is_some(), true);
        assert_eq!(http_response.headers.get("content-type").is_some(), true);
        assert_eq!(http_response.headers["content-type"], "text/html".to_string());
        assert_eq!(http_response.headers["content-length"], "350");
    }

    #[test]
    fn test_http_response_405(){
         let mut file = File::open("tests\\httpHostTestControllerjson.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\httpHostTestControllerjson.txt"),
            Err(e) => panic!("Failed to read tests\\httpHostTestControllerjson.txt, {}", e),
        }

        let root_path = Path::new("C:\\Users\\russ2\\Desktop\\TcpPrograms");
        let http_request =  http_manager::http_parser::http_parser::parse_http_request(buffer, true).unwrap();

        let http_response = http_manager::http_handler::route_handler::http_method_handler(root_path, http_request);

        assert_eq!(http_response.status_code, 405);
        assert_eq!(http_response.status_name, "METHOD NOT ALLOWED".to_string());
        assert_eq!(http_response.headers.get("Allow").is_some(), true);
        assert_eq!(http_response.headers["Allow"], "GET".to_string());


    }
}
