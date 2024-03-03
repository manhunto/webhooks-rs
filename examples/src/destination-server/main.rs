use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();

    println!("Server is listening for requests on {}", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(40..=300);

    println!("Request with response delay {} ms", delay);

    sleep(Duration::from_millis(delay));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}