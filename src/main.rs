// for .read
use std::io::prelude::*;
// for html files
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut line;
    loop {
        line = [0; 512];
        let result = stream.read(&mut line);
        // println!("{}", result.unwrap());
        match result {
            Ok(n) => {
                println!("Request: {}", String::from_utf8_lossy(&line[..]));
                println!("Received {} bytes", n);
                if n < 512 {
                    break;
                }
            },
            _ => {
                println!("here");
            },
        }
    }

    let contents = fs::read_to_string("www/index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    
    // println!("{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
