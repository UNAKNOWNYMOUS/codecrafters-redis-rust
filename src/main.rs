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
            }
            Err(e) => {
                eprint!("Failed to accept connection: {e}");
            }
        }
    }
    Ok(())
}
