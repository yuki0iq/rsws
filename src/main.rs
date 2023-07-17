use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = if request_line.starts_with("GET / HTTP/") {
        ("HTTP/1.1 200 OK", fs::read_to_string("ok.html").unwrap())
    } else {
        ("HTTP/1.1 404 Not Found",
        String::from("<!doctype html><html><head><meta charset=\"utf-8\"><title>error</title></head><body><h1>404 Not Found</h1><hr><i>rust simple web server</i></body></html>"))
    };

    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
