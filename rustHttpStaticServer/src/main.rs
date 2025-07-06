pub(crate) mod http_parser;

use std::{io::{Read, Write}, net::TcpListener, path::Path};
use std::fs::File; 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut counter = 0;
    for stream in listener.incoming(){
        match stream{
            Ok(mut s) => {
                counter+=1;
                let mut buf =[0; 2048];
                s.read(&mut buf).expect("failed to read request");
                let file_name =format!("{}{}{}", "test", counter, ".txt");
                let mut new_file = File::create(Path::new("C:\\Users\\russ2\\Desktop\\TcpPrograms\\rustHttpStaticServer\\src\\HttpParser\\TestRequests").join(file_name )).unwrap();
                //let req =  String::from_utf8_lossy(&mut buf[..]);
                new_file.write_all(&mut buf[..]).unwrap();
                //println!("{}", req);
                s.write(b"Hello World\r\n").unwrap();
                 
                                 
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}
