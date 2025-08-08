use std::{io::{self, BufRead, Read, Write}, net::{Incoming, TcpListener, TcpStream}, thread, time::Duration};

fn main() {
    thread::spawn(move || {
         let listener = TcpListener::bind("127.0.0.1:8899").expect("Failed to connect to server at 127.0.0.1:8899");
         listener.set_nonblocking(true).expect("failed to set not blocking to true");

         loop{
            match listener.accept(){
                Ok(mut message) => {
                    let buf: [u8; 1024] = [0; 1024];
                    let _ = message.0.read(&mut buf.to_vec());

                    match String::from_utf8(buf.to_vec()){
                    Ok(s) => {
                        println!("from {}: {}", message.1, s)
                    },
                    Err(e) => println!("failed to convert message to string {}", e)
                }
                    
                },
                Err(_) => continue
            }
         }
         
    });
    
    let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(user_input) => {
                    let cleaned_user_input = user_input.trim();
                    if cleaned_user_input.len() > 0{
                        let mut listener = TcpStream::connect("127.0.0.1:3333").unwrap();
                        let ff= cleaned_user_input.as_bytes();
                        println!("{:?}", ff);
                        let _ = listener.write(ff);
                    }
                },
                Err(e) => println!("Failed to read in user input {}", e)
            }
        }
    
   

    


}
