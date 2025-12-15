use std::{
    fs,                            // File system operations
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

    /*
        HTTP is a text-based protocol, and a request takes this format:

        Method Request-URI HTTP-Version CRLF
        headers CRLF (carriage return and line feed)
        message-body
    */

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); // HTTP response with status code 200 OK
    //                                                                                               'Success message’s data'
    stream.write_all(response.as_bytes()).unwrap(); // 'as_bytes on our response to convert the string data to bytes. 
    //                                                      The write_all method on stream takes a &[u8] and sends those
    //                                                      bytes directly down the connection'
}
