use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for client in listener.incoming() {
        let client = client.unwrap();

        thread::spawn(|| match handle(client) {
            Ok(_) => println!(""),
            Err(e) => println!("{}", e),
        });
    }
    drop(listener);
    Ok(())
}

fn handle(mut client: TcpStream) -> Result<()> {
    client.set_read_timeout(Some(time::Duration::from_millis(100)))?;
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
