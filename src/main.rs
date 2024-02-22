// let output = Command::new("ls")
//     .args(["-s","-R"])
//     .output()
//     .expect("failed to execute process");

// if output.status.success() {
//     println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
// } else {
//     println!("error: {}", output.status);
// }
// use std::process::Command;

// let path = Path::new("/home/seif"); 

// for entry in WalkDir::new(path).max_depth(1) { 
//     let entry = entry.unwrap();
    
//         println!("{}", entry.path().display());
    
// }
mod server;
mod client;
mod helper;
fn main() {
println!("Hi, Are you the server or the client? 1 for server, 2 for client");
let mut choice = String::new();
std::io::stdin().read_line(&mut choice).unwrap();
let choice_num: u32 ;
match choice.trim().parse(){
    Ok(num) => choice_num = num,
    Err(_) => choice_num = 1,
};

match choice_num {
    1 => {
        println!("You are the server");
        server::main_server();
    }
    2 => {
        println!("You are the client");
        client::main_client();
    }
    _ => println!("Invalid choice"),
}
}