use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("localhost:3000").expect("failed to create TCP listener");

    for connection in listener.incoming() {
        let stream = connection.expect("client connection failed");
        std::thread::spawn(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);

    let mut request = Vec::new();
    reader
        .read_until(b'\n', &mut request)
        .expect("failed to read from stream");

    let request = String::from_utf8(request).expect("malformed request line");
    print!("HTTP request line: {}", request);

    let response = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Length: 12\n",
        "Connection: close\r\n\r\n",
        "Hello world!"
    );

    stream
        .write(response.as_bytes())
        .expect("failed to write to stream");
    stream.flush().expect("failed to flush stream");
}
