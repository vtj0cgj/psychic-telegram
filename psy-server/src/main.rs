use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let nbytes = match stream.read(&mut buffer) {
            Ok(n) if n == 0 => {
                println!("Client disconnected");
                return;
            }
            Ok(n) => n,
            Err(_) => {
                println!("Error reading from stream");
                return;
            },
        };

        let command = String::from_utf8_lossy(&buffer[..nbytes]);
        println!("Received command: {}", command);

        let output = Command::new("sh")
            .arg("-c")
            .arg(command.trim())
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        // Send the output back to the client
        stream.write_all(&output.stdout).expect("Failed to write to stream");

        // Flush the stream to ensure all data is sent
        stream.flush().expect("Failed to flush stream");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:200").expect("Failed to bind port");
    println!("Server listening on port 200...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {:?}", stream.peer_addr());
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
