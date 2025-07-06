#[cfg(test)]
mod tests{
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
}