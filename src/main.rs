use std::{net::TcpListener, io::{Read, Write, BufReader, BufRead}, hash::BuildHasher};

static DBURL: &str = "db";
static TABLE: &str = "translations";

mod http;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Incomming connection: {}", stream.peer_addr()?);
        let mut request_bytes: Vec<u8> = Vec::new();
        let mut read_length = 1;
        let mut buf = [0; 1];
        while read_length != 0 || buf[0] != b'\r' {
            read_length = stream.read(&mut buf)?;
            //print!("{:#?}", buf);
            if buf[0] == b'\r' {
                println!("r")
            }
            if buf[0] == b'\n' {
                println!("n")
            }
            request_bytes.append(&mut buf.to_vec());
        }
        stream.write(String::from("HTTP/2 200 OK").as_bytes())?;
        println!("{:#?}", String::from_utf8_lossy(&request_bytes));
    }
    Ok(())
}
