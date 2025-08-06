use std::{io::Read, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").expect("Failed to connect to server at 127.0.0.1:3333");
    loop{
        for stream in listener.incoming(){
            match stream {
                Ok(mut s) => {
                    let mut temp_buf: [u8; 1024] = [0; 1024];
                    s.read(&mut temp_buf).expect("failed to read request");
                    match String::from_utf8(temp_buf.to_vec()){
                        Ok(r) => println!("{}", r),
                        Err(e) => println!("Cannot convert to string {}", e)
                    }

                },
                Err(e) => println!("failed read incoming messages: {}", e)
            }
        }
    }

}
