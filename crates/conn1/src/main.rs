use std::net::SocketAddr;

use tokio::net::{TcpListener,TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8741").await.unwrap() ; 
    loop {
        let (sockets, Input_socket_addrs) = listener.accept().await.unwrap();


        let handle = tokio::spawn(async move
        {
            process(sockets,Input_socket_addrs).await;        
        }
    )
    }
    // println!("Hello, world!");
}

async fn process(socket : TcpStream, input_socket_addrs : SocketAddr)
{
    let connection = tokio::net::TcpStream::connect(input_socket_addrs).await;
    for meow in socket 
    {
        
    }


}
