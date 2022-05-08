use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    host: String,

    #[clap(short, long)]
    port: i32,
}

fn main() {
    let args = Args::parse();
    let inet = format!("{}:{}", args.host, args.port);
    let mut stream = TcpStream::connect(&inet).unwrap();
    println!("Connected to server {}", inet);

    stream.write(b"hello").unwrap();
    let mut buffer = [0 as u8; 0xFF];
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("read {} bytes, message: {}", size, from_utf8(&buffer).unwrap());
        },
        Err(e) => {
            panic!("Some error occurred: {}", e);
        }
    }
}
