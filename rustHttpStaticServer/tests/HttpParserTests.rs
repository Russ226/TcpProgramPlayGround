#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::{BufReader, Read},
    };

    #[path = "../../src/http_parser/http_parser.rs"]
    mod http_parser;

    #[test]
    fn test_parse_first_line_1() {
        let first_line_header = String::from("GET / HTTP/1.1\r\n");

        let result = http_parser::parse_first_line(&first_line_header).unwrap();

        assert_eq!(result.0, http_parser::HttpMethod::GET);
        assert_eq!(result.1, "/");
        assert_eq!(result.2, "HTTP/1.1");
    }

    #[test]
    fn test_parse_first_line_2() {
        let first_line_header = String::from("POST /test HTTP/1.1\r\n");

        let result = http_parser::parse_first_line(&first_line_header).unwrap();

        assert_eq!(result.0, http_parser::HttpMethod::POST);
        assert_eq!(result.1, "/test");
        assert_eq!(result.2, "HTTP/1.1");
    }
    #[test]
    fn parse_headers_1() {
        let headers = fs::read_to_string("tests\\headers1.txt")
            .expect("Should have been able to read the file");

        let result = http_parser::parse_headers(headers);

        assert_eq!(result["Host"], "127.0.0.1:8080");
        assert_eq!(result["Connection"], "keep-alive");
        assert_eq!(result["Sec-Fetch-User"], "?1");
        assert_eq!(result["Accept-Language"], "en-US,en;q=0.9");
    }

    #[test]
    fn parse_headers_2() {
        let headers = fs::read_to_string("tests\\headers2.txt")
            .expect("Should have been able to read the file");

        let result = http_parser::parse_headers(headers);

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

        let result = http_parser::parse_http_request(buffer).unwrap();

        assert_eq!(result.method, http_parser::HttpMethod::GET);
        assert_eq!(result.route, "/");
        assert_eq!(result.version, "HTTP/1.1");

        assert_eq!(result.headers["Host"], "127.0.0.1:8080");
        assert_eq!(result.headers["Connection"], "keep-alive");
        assert_eq!(result.headers["Sec-Fetch-User"], "?1");
        assert_eq!(result.headers["Accept-Language"], "en-US,en;q=0.9");

        assert_eq!(result.body.len(), 0);
    }
}
