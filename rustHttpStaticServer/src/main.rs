
use std::{
    io::{Read, Write},
    net::TcpListener, thread
};

mod http_parser;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                thread::spawn(move || {
                    let mut buf: Vec<u8> = Vec::new();
                   
                    s.read_to_end(&mut buf).expect("failed to read request");

                    let req = http_parser::http_parser::parse_http_request(buf, true);

                    match req {
                        Ok(r) => println!("{:?}", r),
                        Err(e) => println!("error pasring request\n{}", e)
                    }

                    s.write(b"Hello World\r\n").unwrap();
                });
                
                
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}
