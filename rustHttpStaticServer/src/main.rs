
use std::{
    io::{Read, Write},
    net::TcpListener
};

mod http_parser;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut counter = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                counter += 1;
                let mut buf: Vec<u8> = Vec::new();
                s.read_to_end(&mut buf).expect("failed to read request");
                let req = http_parser::http_parser::parse_http_request(buf);

                match req {
                    Ok(r) => println!("{:?}", r),
                    Err(e) => println!("error pasring request\n{}", e)
                }

                s.write(b"Hello World\r\n").unwrap();
                
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}
