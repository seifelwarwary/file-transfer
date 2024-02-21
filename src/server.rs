

use mime_guess::from_path;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let file_path = "/home/seif/Portfolio/Projects/file-transfer/Cargo.toml";
    let file_name= file_path.split("/").last().unwrap();
    let contents = fs::read(file_path).unwrap();
    let length = contents.len();

    let types=from_path(file_name).first_raw().unwrap();

    let response = format!("{status_line}\r\n\
    Content-Disposition: attachment; filename=\"{file_name}\"\r\n\
    Content-Type: {types}\r\n\
    Content-Length: {length}\r\n\r\n");
        
    stream.write_all(&response.as_bytes()).unwrap();
    match stream.write_all(&contents){
        Ok(_) => println!("ok"),
        Err(e) => println!("{}",e)
    };
    stream.flush().unwrap(); 
}
pub fn main_server() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}