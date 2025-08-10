use std::{io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, sync::mpsc, thread};

mod model;
extern crate message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").expect("Failed to connect to server at 127.0.0.1:3333");
    let mut addrs: Vec<model::chat_user::ChatUser> = Vec::new();
    
    loop{
        match listener.accept() {
            Ok(mut message) => {
                
                thread::spawn(move || {
                    let (sender, receiver) = mpsc::channel::<message::message::Message>();
                    // parse message
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
                            let mut body:Vec<u8> = Vec::with_capacity(m.size as usize);

                            let mut counter = 0; 
                            while counter < m.size {
                                let mut temp_buf: [u8; 1] = [0; 1];
                                message.0.read(&mut temp_buf).expect("failed to read request");

                                body.push(temp_buf[0]);
                                counter+=1;
                            }

                            match String::from_utf8(body) {
                                Ok(sb) => {
                                    m.message_body = sb;
                                    sender.send(m);
                                },
                                Err(e) => println!("Unable to parse body of message from {} with name {} with error {}", m.sender_ip, m.sender_display_name, e),
                            }
                        },
                        None => {
                            println!("Failed to parse incoming message");
                        },
                    }

                    // check if new user


                });

            },
            Err(e) => println!("failed read message {}", e)
            
        }

        match receiver.try_recv(){
            Ok(s) => {
                for &addr in &addrs{
                    //let mut send_messsage_conn = TcpStream::connect(addr).expect(&format!("failed to send message to {}", addr));
                    //let _ = send_messsage_conn.write(s.as_bytes());
                    //let _ = send_messsage_conn.shutdown(std::net::Shutdown::Both);
                }
            },
            Err(e) => println!("Error sending message to clients {}", e)
        }
    }
}


