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
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // Reading the first line of the HTTP request

    /*
        HTTP is a text-based protocol, and a request takes this format:

        Method Request-URI HTTP-Version CRLF
        headers CRLF (carriage return and line feed)
        message-body
    */

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents: String = fs::read_to_string(filename).unwrap(); // Reading the content of the requested file  
    let length = contents.len(); // Getting the length of the file content
 
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); // HTTP response with status code 200 OK
    //                                                                                                'Success message’s data'
    stream.write_all(response.as_bytes()).unwrap(); // Sending the response back to the client
    //                                                      'as_bytes on our response to convert the string data to bytes.
    //                                                      The write_all method on stream takes a &[u8] and sends those
    //                                                      bytes directly down the connection'
}
