use std::{io::{Read, Write}, net::TcpListener}; 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        match stream{
            Ok(mut s) => {
                let mut buf =[0; 1024];
                 s.read(&mut buf).expect("failed to read request");
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
