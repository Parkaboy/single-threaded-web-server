use std::net::TcpListener;

fn main() {
    // Bind the TCP listener to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // Handle the connection
                println!("New connection established!");
            }
            Err(e) => {
                // Handle the error
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}