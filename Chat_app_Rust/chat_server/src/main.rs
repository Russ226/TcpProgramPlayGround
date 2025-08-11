use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc},
    thread,
};

use crate::model::chat_user;

mod model;
extern crate message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").expect("Failed to connect to server at 127.0.0.1:3333");
    let mut addrs: Arc<Vec<model::chat_user::ChatUser>> = Arc::new(Vec::new());
    let (sender, receiver) = mpsc::channel::<message::message::Message>();

    loop {
        let sender_cl = sender.clone();
        
        match listener.accept() {
            Ok(mut message) => {
                thread::spawn(move || {
                    let mut buf: Vec<u8> = Vec::new();
                    loop {
                        let mut temp_buf: [u8; 1] = [0; 1];
                        message
                            .0
                            .read(&mut temp_buf)
                            .expect("failed to read request");

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

                                    match sender_cl.send(m) {
                                        Ok(_) => (),
                                        Err(e) => println!("Error dispatching message, {}", e),
                                    }
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
                });

            }
            Err(e) => println!("failed read message {}", e),
        }

        
        match receiver.try_recv() {
            Ok(s) => {
                let chat_user = chat_user::ChatUser::new(s.sender_ip.clone(),s.sender_display_name.clone(),);
                if !addrs.contains(&chat_user) {
                    Arc::make_mut(&mut addrs).push(chat_user);
                }
                for addr in &*addrs {
                    match TcpStream::connect(addr.ip_addr.clone()) {
                        Ok(mut stream) => {
                            let _ = stream.write(&s.clone().convert_to_u8());
                            let _ = stream.shutdown(std::net::Shutdown::Both);
                        },
                        Err(_) => {
                            println!("Could send message to {} {}", addr.ip_addr, addr.username);
                            // remove from array 
                        },
                    }

                }
            }
            Err(e) => println!("Error sending message to clients {}", e)
        }
    }
}
