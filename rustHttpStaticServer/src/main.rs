use std::{
    io::{Read},
    net::TcpListener,
    thread,
};
use http_manager;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                thread::spawn(move || {
                    let mut buf: Vec<u8> = Vec::new();

                    loop {
                        let mut temp_buf: [u8; 1] = [0; 1];
                        s.read(&mut temp_buf).expect("failed to read request");

                        buf.push(temp_buf[0]);

                        let mut last_4_char: Vec<u8> = Vec::new();

                        for n in buf.iter().rev().take(4) {
                            last_4_char.insert(0, *n);
                        }

                        match String::from_utf8(last_4_char) {
                            Ok(s) => {
                                if s == "\r\n\r\n" {
                                    break;
                                }
                            }
                            Err(e) => {
                                println!("cannot read string from incoming messaing,\n{}", e);
                                break;
                            }
                        };
                    }

                    match http_manager::http_parser::http_parser::parse_http_request(buf, false) {
                        Ok(mut r) => {
                            if r.headers.contains_key("Content-Length"){
                                match r.headers["Content-Length"].parse::<u64>() {
                                    Ok(size) => {
                                        let mut body:Vec<u8> = Vec::with_capacity(size as usize);

                                        let mut counter = 0; 
                                        while counter < size {
                                            let mut temp_buf: [u8; 1] = [0; 1];
                                            s.read(&mut temp_buf).expect("failed to read request");

                                            body.push(temp_buf[0]);
                                            counter+=1;
                                        }

                                        r.body = body;
                                    },
                                    Err(e) => println!("unable to parse content-length {}", e),
                                }
                            }
                            println!("{:?}", r);
                        }
                        Err(e) => println!("error pasring request\n{}", e),
                    }

                   // s.write(b"Hello World\r\n").unwrap();
                });
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}
