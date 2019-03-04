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
    let mut request = vec![0; 1024];
    client.read(&mut request)?;

    let mut server = TcpStream::connect("127.0.0.1:9000")?;
    server.write_all(&request)?;
    server.shutdown(std::net::Shutdown::Write)?;

    let mut response = Vec::new();
    server.read_to_end(&mut response)?;

    client.write_all(&response)?;
    Ok(())
}
