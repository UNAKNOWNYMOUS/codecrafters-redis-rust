#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    println!("Listening on: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Got connection from: {}", stream.peer_addr()?);
                handle_client(stream)?;
            }
            Err(e) => {
                eprint!("Failed to accept connection: {e}");
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];
    loop {
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            println!("Client disconnected!");
            return Ok(());
        }

        stream.write_all(b"+PONG\r\n")?;
    }
}
