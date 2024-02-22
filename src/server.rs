
use  crate::helper::*;
use mime_guess::from_path;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, path::Path,
};


pub fn main_server() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    write_to_cmd("Server listening on port 7878\n");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut current_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        loop{
        let mut  dirs;
        let buffer=&mut [0; 1024];
        stream.read(buffer).unwrap();
        let  command = String::from_utf8_lossy(buffer).to_string();
        let command=command.trim_end_matches('\0');
        let mut commands_vec:Vec<&str>=command.split(" ").collect();
        let command = commands_vec[0];
        match command {
            "ls" => {
                 dirs=read_dir(&current_dir);
                 stream.write_all(format_dirs(&dirs).as_bytes()).unwrap();
            }
            "cd" => {
                let path;
                let dir_holder=current_dir;
                let path1 = Path::new(&dir_holder);
                let path2 = Path::new(commands_vec[1]);
                let joined_path = path1.join(path2);
                if commands_vec[1] == ".." {
                    path = path1.parent().unwrap();
                } else {
                    path = joined_path.as_ref();
                }
                current_dir=path.to_str().unwrap().to_string();
                dirs = read_dir(path.to_str().unwrap());
                stream.write_all(format_dirs(&dirs).as_bytes()).unwrap();
            }
            // "download" => {
            //     write_to_cmd("download");
            //     download_file(stream);
            // }
            // "upload" => {
            //     write_to_cmd("upload");
            //     let mut file = std::fs::File::open("file.txt").unwrap();
            //     let mut buffer = [0; 1024];
            //     loop {
            //         let bytes_read = file.read(&mut buffer).unwrap();
            //         if bytes_read == 0 {
            //             break;
            //         }
            //         stream.write_all(&buffer[..bytes_read]).unwrap();
            //     }
            //     println!("File sent");
            // }
            "exit" => {
                write_to_cmd("exit\n");
                break;
            }
            _ => {
                match stream.write_all("Invalid command\n".as_bytes()){
                    Ok(_) => {},
                    Err(_) => {write_to_cmd("Error writing to stream\n");}
                };
            }
        }
       }
    }

}