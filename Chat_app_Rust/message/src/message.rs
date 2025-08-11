
use std::fmt::{self, Display};
use regex::RegexBuilder;

/*
    structure of message 
    header seperated by white space and ends with double \n

    size sender_ip sender_display_name\n\n
    message

*/
//todo!(add uri encoding to dipslay_name and message_body)

#[derive(Debug, Clone)]
pub struct Message{
    pub size: usize,
    pub sender_ip: String,
    pub sender_display_name: String,
    pub message_body: String
}

impl Message{
    pub fn new(size: usize, sender_ip: String, sender_display_name: String, message_body: String) -> Message {
        return Message{size, sender_ip, sender_display_name, message_body};
    }

    pub fn convert_to_u8(&self) -> Vec<u8>{
        let str_rep: String = format!("{} {} {}\r\n\r\n{}", self.size, self.sender_ip, self.sender_display_name, self.message_body);

        return str_rep.as_bytes().to_vec();

    }
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.sender_display_name, self.message_body)
    }
}

pub fn str_to_message(str_message: String) -> Option<Message> {
    let split_header_body: Vec<_> = str_message.split("\r\n\r\n").collect();

    if split_header_body.len() == 2 {
        let split_header: Vec<_> = split_header_body[0].split(" ").collect();
        if split_header.len() == 3 {
            let size: usize = match split_header[0].parse() {
                Ok(s) => s,
                Err(e) => {
                    println!("error parsing size of message {}", e);
                    0
                }
            };
            let re = RegexBuilder::new(r"([\d]{1,3}\.[\d]{1,3}\.[\d]{1,3}\.[\d]{1,3}):([\d]{1,5})")
                .build().expect("faiiled to create regex pattern to validate ip address");

            if !re.is_match(split_header[1]){
                return None;
            }

            return Some(Message {size: size, sender_ip: split_header[1].to_string(), 
                sender_display_name: split_header[2].to_string(), message_body: split_header_body[1].to_string() });

        }

            
    }
    return None
}

pub fn u8_to_message(u8_message: Vec<u8>) -> Option<Message> {
    return match String::from_utf8(u8_message) {
        Ok(s) => str_to_message(s),
        Err(e) => {
            println!("Failed to parse messsage from u8 {}", e);
            return None;
        },
    }
}

pub fn str_to_message_header(str_message: String) -> Option<Message> {
    let split_header_body = str_message.replace("\r\n\r\n", "");

    let split_header: Vec<_> = split_header_body.split(" ").collect();
        if split_header.len() == 3 {
            let size: usize = match split_header[0].parse() {
                Ok(s) => s,
                Err(e) => {
                    println!("error parsing size of message {}", e);
                    0
                }
            };
            let re = RegexBuilder::new(r"([\d]{1,3}\.[\d]{1,3}\.[\d]{1,3}\.[\d]{1,3}):([\d]{1,5})")
                .build().expect("faiiled to create regex pattern to validate ip address");

            if !re.is_match(split_header[1]){
                return None;
            }

            return Some(Message {size: size, sender_ip: split_header[1].to_string(), 
                sender_display_name: split_header[2].to_string(), message_body: String::new() });

        }
    return None
}

pub fn u8_to_message_header(u8_message: Vec<u8>) -> Option<Message> {
    return match String::from_utf8(u8_message) {
        Ok(s) => str_to_message_header(s),
        Err(e) => {
            println!("Failed to parse messsage from u8 {}", e);
            return None;
        },
    }
}