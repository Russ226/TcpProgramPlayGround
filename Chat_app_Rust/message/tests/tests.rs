#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};
    extern crate message;

    #[test]
    fn test_parse_message(){

        let mut file = File::open("C:\\Users\\russ2\\Desktop\\TcpPrograms\\Chat_app_Rust\\message\\tests\\message1.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\message1.txt"),
            Err(e) => panic!("Failed to read tests\\message1.txt, {}", e),
        }

        match message::message::u8_to_message(buffer){
            Some(m) =>{
                assert_eq!(m.size, 28);
                assert_eq!(m.sender_display_name, "test1".to_string());
                assert_eq!(m.sender_ip, "127.0.0.1:8899".to_string());
                assert_eq!(m.message_body, "this is a test message".to_string());
            },
            None => assert_eq!(false, true),
        }



    }
}