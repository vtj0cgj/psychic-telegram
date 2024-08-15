use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    println!("Welcome to the Server Selector!\nPress any button to continue.");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Server IP:");
    let mut server_ip = String::new();
    io::stdin().read_line(&mut server_ip).expect("Failed to read line");
    let server_ip = server_ip.trim();

    println!("Server port:");
    let mut server_port = String::new();
    io::stdin().read_line(&mut server_port).expect("Failed to read line");
    let server_port = server_port.trim();

    println!("\nIP: {}\nPort: {}", server_ip, server_port);

    let server_address = format!("{}:{}", server_ip, server_port);
    match TcpStream::connect(&server_address) {
        Ok(mut stream) => {
            println!("Connected to server!");
            loop {
                // Get the command from the user
                print!("> ");
                io::stdout().flush().unwrap();

                let mut command = String::new();
                io::stdin().read_line(&mut command).expect("Failed to read line");

                // Send the command to the server
                stream.write_all(command.as_bytes()).expect("Failed to write to server");

                // Read the response from the server
                let mut buffer = [0; 512];
                let mut response = String::new();
                while let Ok(n) = stream.read(&mut buffer) {
                    if n == 0 {
                        break; // The server closed the connection
                    }
                    response.push_str(&String::from_utf8_lossy(&buffer[..n]));
                    if n < buffer.len() {
                        break; // End of response
                    }
                }

                // Output the server response
                println!("{}", response);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}
