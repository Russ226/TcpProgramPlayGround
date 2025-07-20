pub(crate) mod http_parser;

use std::fs::File;
use std::{
    io::{Read, Write},
    net::TcpListener,
    path::Path,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut counter = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                counter += 1;
                let mut buf: Vec<u8> = Vec::new();
                s.read_to_end(&mut buf).expect("failed to read request");
                let req = String::from_utf8_lossy(&mut buf[..]);

                println!("{}", req);
                s.write(b"Hello World\r\n").unwrap();
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}
