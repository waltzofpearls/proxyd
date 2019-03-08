use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for client in listener.incoming() {
        handle(client?)?;
    }
    Ok(())
}

fn handle(mut client: TcpStream) -> Result<()> {
    client.set_read_timeout(Some(Duration::from_millis(100)))?;
    let mut server = TcpStream::connect("127.0.0.1:9000")?;
    loop {
        let mut buf = [0];
        let _ = match client.read(&mut buf) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => {
                    break;
                }
                _ => return Err(e),
            },
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                server.write(&mut buf)?;
            }
        };
    }

    server.shutdown(std::net::Shutdown::Write)?;

    let mut response = Vec::new();
    server.read_to_end(&mut response)?;

    client.write_all(&response)?;
    Ok(())
}
