use std::{io::{self, BufRead, Write}, net::{Incoming, TcpListener, TcpStream}};

fn main() {
    let stdin = io::stdin();
    let listener = TcpStream::connect("127.0.0.1:8080").unwrap();
    
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                
            },
            Err(e) => println!("Failed to read in user input {}", e)
        }
    }
}
