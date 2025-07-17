#[cfg(test)]
mod tests{
    use std::fs;

    #[path = "../../src/http_parser/http_parser.rs"]
    mod http_parser;

    #[test]    
    fn test_parse_first_line_1(){
        let first_line_header = "GET / HTTP/1.1\r\n";

       let result = http_parser::parse_first_line(first_line_header).unwrap();

       assert_eq!(result.0, http_parser::HttpMethod::GET);
       assert_eq!(result.1, "/");
       assert_eq!(result.2, "HTTP/1.1");
       
    }

    #[test]
    fn test_parse_first_line_2(){
        let first_line_header = "POST /test HTTP/1.1\r\n";

       let result = http_parser::parse_first_line(first_line_header).unwrap();

       assert_eq!(result.0, http_parser::HttpMethod::POST);
       assert_eq!(result.1, "/test");
       assert_eq!(result.2, "HTTP/1.1");
       
    }
    #[test]
    fn parse_headers(){
        let headers = fs::read_to_string("tests\\headers1.text").expect("Should have been able to read the file");

        let result = http_parser::parse_headers(&headers);

        assert_eq!(result["Host"], "127.0.0.1:8080");
        assert_eq!(result["Connection"], "keep-alive");
        assert_eq!(result["Sec-Fetch-User"], "?1");
        assert_eq!(result["Accept-Language"], "en-US,en;q=0.9");
    }
}