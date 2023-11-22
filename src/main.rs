use std::{net::TcpListener, io::{Read, Write, BufReader, BufRead}};

static DBURL: &str = "db";
static TABLE: &str = "translations";

mod http;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Incomming connection: {}", stream.peer_addr()?);
        let buf_reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        for line in buf_reader.lines() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            buffer.push(line);
        }
        println!("{}", buffer.join("\n"));
        stream.write(String::from("HTTP/2 200 OK").as_bytes())?;
    }
    Ok(())
}
