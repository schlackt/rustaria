use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Read message");

    let mut buffer = [0u8; 15];
    stream.read_exact(&mut buffer).unwrap();

    println!("Processing messages");

    let length = buffer[0] as u16 | ((buffer[1] as u16) << 8);
    let message_type = buffer[2];

    println!("--- message received ---");
    dbg!(length);
    dbg!(message_type);

    println!("{}", String::from_utf8(buffer[3..].to_vec()).unwrap());

    println!("Request: {:#?}", buffer);
}