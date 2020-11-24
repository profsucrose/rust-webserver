use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::path::Path;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET";
    if !buffer.starts_with(get) {
        let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return
    } 

    let mut filepath = String::new();
    for &c in buffer.iter().skip(4) {
        println!("c: {}", c as char);
        if c == (' ' as u8) {
            if filepath.as_bytes()[filepath.len() - 1] == b'/' {
                filepath.push_str("index");
            }
            break;
        }
        filepath.push(c as char);
    }

    if Path::new(&format!("pages{}", filepath)).exists()
            && !Path::new(&format!("pages/{}.html", filepath)).exists() {
        filepath.push_str("/index");
    }

    println!("Filepath: {}", filepath);
    let contents = fs::read_to_string(get_page_path(filepath)).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_page_path(filename: String) -> String {
    format!("pages/{}.html", filename)
}