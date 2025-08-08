use std::{io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, sync::mpsc};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").expect("Failed to connect to server at 127.0.0.1:3333");
    let mut addrs: Vec<SocketAddr> = Vec::new();
    let (sender, receiver) = mpsc::channel::<String>();
    loop{
        match listener.accept() {
            Ok(mut message) => {
                
                if !addrs.contains(&message.1) {
                    addrs.push(message.1.clone());
                }
                let mut buf: [u8; 1024] = [0; 1024];
                match message.0.read(&mut buf){
                    Ok(size) => println!("size of message from {} is {}", message.1, size),
                    Err(e) => println!("failed to read message from {} with error {}", message.1, e)
                };

                match String::from_utf8(buf.to_vec()){
                    Ok(s) => {
                        let _ = sender.send(s.clone());
                        println!("{}", s);
                    },
                    Err(e) => println!("failed to convert message to string {}", e)
                }

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


