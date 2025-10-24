use std:: {
    io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};


fn main() {
    // Bind the TCP listener to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // Handle the connection
                handle_connection(_stream);
            }
            Err(e) => {
                // Handle the error
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);

    let http_request: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .take_while(|line| !line.is_empty())
        .collect();

    // Print the request lines
    println!("Request:\n{}", http_request.join("\n"));

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
}