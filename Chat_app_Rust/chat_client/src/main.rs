use std::{io::{self, BufRead, Read, Write}, net::{Incoming, TcpListener, TcpStream}, thread, time::Duration};

extern crate message;
fn main() {
    thread::spawn(move || {
        
         let listener = TcpListener::bind("127.0.0.1:8899").expect("Failed to connect to server at 127.0.0.1:8899");
         //listener.set_nonblocking(true).expect("failed to set not blocking to true");

         loop{
            match listener.accept(){
                Ok(mut message) => {
                    let mut buf: Vec<u8> = Vec::new();
                    
                    loop {
                        let mut temp_buf: [u8; 1] = [0; 1];
                        message.0.read(&mut temp_buf).expect("failed to read request");
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
                                println!("cannot read string from incoming message,\n{}", e);
                                break;
                            }
                        };
                    }

                    match message::message::u8_to_message_header(buf) {
                        Some(mut m) => {
                            let mut body: Vec<u8> = Vec::with_capacity(m.size as usize);

                            let mut counter = 0;
                            while counter < m.size {
                                let mut temp_buf: [u8; 1] = [0; 1];
                                message.0.read(&mut temp_buf).expect("failed to read request");

                                body.push(temp_buf[0]);
                                counter += 1;
                            }

                            match String::from_utf8(body) {
                                Ok(sb) => {
                                    m.message_body = sb;
                                    println!("{}", m);
                                }
                                Err(e) => println!(
                                    "Unable to parse body of message from {} with name {} with error {}",
                                    m.sender_ip, m.sender_display_name, e
                                ),
                            }
                        }
                        None => {
                            println!("Failed to parse incoming message");
                        }
                    }
                    
                },
                Err(e) => println!("error recieving message {}", e)
            }
         }
         
    });
    
    let stdin = io::stdin();
        for line in stdin.lines() {
            match line {
                Ok(user_input) => {
                    let cleaned_user_input = user_input.trim();
                    if cleaned_user_input.len() > 0{
                        let send_message = message::message::Message::new(cleaned_user_input.len(), "127.0.0.1:8899".to_string(), 
                                    "test1".to_string(), cleaned_user_input.to_string());
                        let mut listener = TcpStream::connect("127.0.0.1:3333").unwrap();
                        let ff= message::message::Message::convert_to_u8(&send_message);
                        let _ = listener.write(&ff);
                    }
                },
                Err(e) => println!("Failed to read in user input {}", e)
            }
        }
    
   

    


}
