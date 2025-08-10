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
    #[test]
    fn test_parse_partial_faulty_message(){

        let mut file = File::open("C:\\Users\\russ2\\Desktop\\TcpPrograms\\Chat_app_Rust\\message\\tests\\faulty_improper_size_message.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\faulty_improper_size_message.txt"),
            Err(e) => panic!("Failed to read tests\\faulty_improper_size_message.txt, {}", e),
        }

        match message::message::u8_to_message(buffer){
            Some(m) =>{
                assert_eq!(m.size, 0);
                assert_eq!(m.sender_display_name, "test1".to_string());
                assert_eq!(m.sender_ip, "127.0.0.1:8899".to_string());
                assert_eq!(m.message_body, "this is a test message".to_string());
            },
            None => assert_eq!(false, true)
        }
    
    }

    #[test]
    fn test_parse_missing_disp_name(){
        let mut file = File::open("C:\\Users\\russ2\\Desktop\\TcpPrograms\\Chat_app_Rust\\message\\tests\\missing_display_name.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\missing_display_name.txt"),
            Err(e) => panic!("Failed to read tests\\missing_display_name.txt, {}", e),
        }

         match message::message::u8_to_message(buffer){
            Some(_) => assert_eq!(false, true),
            None => assert_eq!(true, true)
        }
    }

    #[test]
    fn test_parse_bad_ip_addr(){
        let mut file = File::open("C:\\Users\\russ2\\Desktop\\TcpPrograms\\Chat_app_Rust\\message\\tests\\bad_ip_addr.txt")
            .expect("Should have been able to read the file");
        let mut buffer: Vec<u8> = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => println!("Finished reading tests\\bad_ip_addr.txt"),
            Err(e) => panic!("Failed to read tests\\bad_ip_addr.txt, {}", e),
        }

         match message::message::u8_to_message(buffer){
            Some(_) => assert_eq!(false, true),
            None => assert_eq!(true, true)
        }
    }
}