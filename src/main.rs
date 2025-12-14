use std::{
    io::{BufReader, prelude::*}, // Importing the necessary traits for reading and writing to the TCP stream
    net::{TcpListener, TcpStream}, // Transmission Control Protocol (link between two devices)
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:2002").unwrap(); // TCP listener bound to localhost on port 2002
    //                                                                                 'bind' is the networking 'new'
    //                                                                                 'unwrap' is used to handle errors (in this case, it will panic if the binding fails)
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader // Reading the HTTP request from the stream from client 2 server
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
