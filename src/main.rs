use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};
use std::fmt::{Debug, Formatter};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let message = read_next_message(&mut stream);
    println!("{:?}", message);

    stream.write(&[2u8,0,3u8,0]);

    let message = read_next_message(&mut stream);
    println!("{:?}", message);

    let message = read_next_message(&mut stream);
    println!("{:?}", message);
}

fn read_next_message(stream: &mut TcpStream) -> TerrariaMessage {
    println!("Read message");

    let mut header_buffer = [0u8; 2];
    stream.read_exact(&mut header_buffer).unwrap();

    println!("Processing messages");

    let length = header_buffer[0] as u16 | ((header_buffer[1] as u16) << 8);
    let payload_length = (length - 2) as usize;
    let mut payload_buffer = vec![0; payload_length];
    let bytes_read = stream.read(payload_buffer.as_mut_slice()).unwrap();
    let message_type = payload_buffer[0];

    TerrariaMessage { kind: TerrariaMessageKind::from_u8(message_type),  payload: payload_buffer[1..].to_vec()}
}

struct TerrariaMessage {
    kind: TerrariaMessageKind,
    payload: Vec<u8>
}

impl Debug for TerrariaMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Ok(string) = String::from_utf8(self.payload.clone()) {
            f.write_fmt(format_args!("kind = {:?}, payload = {}", self.kind, string.trim()))
        } else {
            f.write_fmt(format_args!("kind = {:?}, payload = {:?}", self.kind, self.payload))
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
enum TerrariaMessageKind {
    ConnectRequest = 1u8,
    PlayerInfo = 4u8,
    ClientUUID = 68u8
}

impl TerrariaMessageKind {
    fn from_u8(value: u8) -> Self {
        match value {
            1u8 => Self::ConnectRequest,
            4u8 => Self::PlayerInfo,
            68u8 => Self::ClientUUID,
            _ => panic!("Invalid message type {}", value)
        }
    }
}