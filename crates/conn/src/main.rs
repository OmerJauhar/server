// use core::prelude;
use std::net::{TcpListener, TcpStream} ; 
use std::io::{prelude::*, BufReader};
fn main() {
    // println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap(); 
    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        println!("Connection established ");
        handle_connection(stream);
    }
}
fn handle_connection(mut stream :TcpStream)
{
    let  buffer = BufReader::new(&mut stream) ; 
    let http_requests : Vec<_> = buffer
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();

    println!("Requests : {:#?}",http_requests);
}
//format of the response 
/*HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body */
