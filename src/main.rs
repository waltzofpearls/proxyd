use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for client in listener.incoming() {
        handle(client?)?;
    }
    Ok(())
}

fn handle(mut client: TcpStream) -> Result<()> {
    let mut server = TcpStream::connect("127.0.0.1:9000")?;
    server.write(&[1])?;
    server.read(&mut [0; 128])?;
    // client.write(&[1])?;
    // client.read(&mut [0; 128])?;
    client.write_all(b"HTTP/1.0 200 OK\r\n\r\n").unwrap();
    Ok(())
}
