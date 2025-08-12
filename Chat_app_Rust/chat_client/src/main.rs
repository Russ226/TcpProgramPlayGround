use core::time;
use std::{env, fs::File, io::{self,Read, Write}, net::{TcpListener, TcpStream}, thread};
use serde::Deserialize;
extern crate message;

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientConfig{
    ip_address: String,
    host_ip_address: String,
    user_name: String,
    user_input: bool,
    messages: Vec<String>
}

fn run(config: ClientConfig){
    let ipadd = config.ip_address.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind(ipadd).expect("Failed to connect to server at 127.0.0.1:8899");
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

    if config.user_input {
        let stdin = io::stdin();
        for line in stdin.lines() {
            match line {
                Ok(user_input) => {
                    let cleaned_user_input = user_input.trim();
                    if cleaned_user_input.len() > 0{
                        let send_message = message::message::Message::new(cleaned_user_input.len(), config.ip_address.clone(), 
                                    config.user_name.clone(), cleaned_user_input.to_string());
                        let mut listener = TcpStream::connect(config.host_ip_address.clone()).unwrap();
                        let ff= message::message::Message::convert_to_u8(&send_message);
                        let _ = listener.write(&ff);
                    }
                },
                Err(e) => println!("Failed to read in user input {}", e)
            }
        }
    }
    else {
        for message in config.messages {
            let send_message = message::message::Message::new(message.len(), config.ip_address.clone(), 
                                    config.user_name.clone(), message);
            let mut listener = TcpStream::connect(config.host_ip_address.clone()).unwrap();
            let ff= message::message::Message::convert_to_u8(&send_message);
            let _ = listener.write(&ff);
            thread::sleep(time::Duration::from_millis(1000));
        }

        thread::sleep(time::Duration::from_millis(3000));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("comand args {:?}", args);

    let mut file = File::open(&args[1])
            .expect("Should have been able to read the file");
    let mut buffer: Vec<u8> = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(_) => println!("Finished reading config.json"),
        Err(e) => panic!("Failed to read config.json, {}", e),
    }

    match serde_json::from_slice::<ClientConfig>(&buffer){
        Ok(c) => {
            println!("{:?}", c);
            run(c);
        },
        Err(e) => panic!("failed to read config {}", e)
    }
}
