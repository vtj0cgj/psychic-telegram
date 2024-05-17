use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    println!("Welcome to the Server Selector!\nPress any button to continue.");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Server IP:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let server_ip: String = input;

    println!("Server port:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let server_port: String = input;

    println!("\nIP: {}Port: {}", server_ip, server_port);

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Connected to server!");
            loop {
                print!("> ");
                io::stdout().flush().unwrap();

                let mut command = String::new();
                io::stdin().read_line(&mut command).expect("Failed to read line");

                stream.write_all(command.as_bytes()).expect("Failed to write to server");

                let mut buffer = [0; 512];
                stream.read(&mut buffer).expect("Failed to read from server");
                let output = String::from_utf8_lossy(&buffer);
                println!("{}", output);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}