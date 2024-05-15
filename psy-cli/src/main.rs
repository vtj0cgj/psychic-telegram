use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    println!("Welcome to the Server Selector!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let server_address = input;

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