use clap::Parser;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    host: String,

    #[clap(short, long)]
    port: i32,
}

/// simple echo server
fn main() {
    let args = Args::parse();
    println!("Server Starts, {:?}", args);

    let listener = TcpListener::bind(format!("{}:{}", args.host, args.port)).unwrap();
    println!("Server listening on port {} ...", args.port);

    let mut buffer = [0 as u8; 0xFF];
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                loop {
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            // echo back
                            stream.write(&buffer[0..size]).unwrap();
                        }
                        Err(_) => {
                            stream.shutdown(Shutdown::Both).unwrap();
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    drop(listener);
}
