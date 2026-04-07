#![allow(unused_imports)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Client: {} connected!", addr);

        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0u8; 1024];

    loop {
        let bytes_read = match socket.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        socket.write_all(b"+PONG\r\n").await.unwrap();
    }
}
