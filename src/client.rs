use  crate::helper::*;

use std::{
    
    io::{prelude::*, stdin}, net::TcpStream, str::FromStr, thread, time::Duration
};

pub fn main_client() {
    write_to_cmd("Enter the address:port of the server you want to connect to:");
    let mut ip = String::from_str("127.0.0.1:7878").unwrap();
    
    // stdin().read_line(&mut ip).unwrap();
    write_to_cmd("127.0.0.1:7878");
    let mut stream = TcpStream::connect(ip.trim()).unwrap();
    
    
    loop{
        write_to_cmd("Enter your command: ls, cd, download, upload or exit\n");
        let mut command = String::new();
        write_to_cmd("");
        stdin().read_line(&mut command).unwrap();
        let mut command=&command[..command.len()-1];
        if command.is_empty(){
            command="\0";
        }
        
        stream.write_all(command.as_bytes()).unwrap();
        thread::sleep(Duration::from_millis(10));
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer);
        write_to_cmd(&response.trim());
        if command.trim() == "exit" {
            break;
        }
    }

}
