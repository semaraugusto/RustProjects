use std::net::TcpListener;
use std::net::TcpStream;
use std::io::*;
use std::fs;
use std::thread;
use std::time::Duration;
use std::str;
// use lib::ThreadPool;
// pub mod src::ThreadPool;
mod lib;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = lib::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get_root_route = b"GET / HTTP/1.1\r\n";
    let get_sleep_route = b"GET /sleep HTTP/1.1\r\n";

    println!("{}", String::from_utf8_lossy(&buffer));
    let (status_line, filename) = if buffer.starts_with(get_root_route) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(get_sleep_route) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    send_response(stream, status_line, &contents);
}

fn send_response(mut stream: TcpStream, status_line: &str, contents: &str) {
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
