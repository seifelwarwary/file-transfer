use std::{collections::HashMap, fs, io::{stdout, Read, Write}, net::TcpStream, path::Path};

use mime_guess::from_path;
use walkdir::WalkDir;

static MB: f32 = 1024.0 * 1024.0 ;

pub fn handle_download(mut stream: TcpStream) {
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
pub fn read_dir(dir: &str) -> HashMap<String,u64> {
    let path = Path::new(dir);
    let mut files = HashMap::new();
    for entry in WalkDir::new(path).max_depth(1) {
        let entry = entry.unwrap();
        let size=entry.metadata().unwrap().len();
        let path = entry.path().to_str().unwrap().to_string();
        if path!=dir{
           files.insert(path, size);
        }
    }
    
    files
}
pub fn format_dirs(hashmap : &HashMap<String,u64>) -> String {
    let mut dirs = String::new();
    for (key, value) in hashmap {
        dirs.push_str(&format!("{}: {} MB\n", key, (*value as f32)/MB));
    }
    dirs
}

pub fn write_to_cmd( command:&str){
    stdout().write_all(format!("file-transfer>{command}").as_bytes()).unwrap();

}

pub fn download_file(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut file = std::fs::File::create("file.txt").unwrap();
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read]).unwrap();
    }
    println!("File received");
}