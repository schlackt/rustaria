use std::{
    io::{prelude::*},
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

    let mut header_buffer = [0u8; 3];
    stream.read_exact(&mut header_buffer).unwrap();

    println!("Processing messages");

    let length = header_buffer[0] as u16 | ((header_buffer[1] as u16) << 8);
    let message_type = header_buffer[2];

    let payload_length = (length - 3) as usize;
    let mut payload_buffer = vec![0; payload_length];
    let bytes_read = stream.read(payload_buffer.as_mut_slice()).unwrap();

    dbg!(bytes_read);

    println!("--- message received ---");
    dbg!(length);
    dbg!(message_type);

    println!("{}", String::from_utf8(payload_buffer).unwrap());
}