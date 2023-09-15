use std::{
    io::{Cursor, prelude::*},
    net::{TcpListener, TcpStream},
};
use std::fmt::{Debug, Formatter};
use num_enum::TryFromPrimitive;
use byteorder::{LittleEndian, ReadBytesExt};

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

    stream.write_all(&[2u8, 0, 3u8, 0]).unwrap();

    let message = read_next_message(&mut stream);
    println!("{:?}", message);

    let message = read_next_message(&mut stream);
    println!("{:?}", message);
}

fn read_next_message(stream: &mut TcpStream) -> TerrariaMessage {
    let mut header_buffer = [0u8; 2];
    stream.read_exact(&mut header_buffer).unwrap();

    let length = Cursor::new(header_buffer).read_u16::<LittleEndian>().unwrap();
    let payload_length = (length - 2) as usize;
    let mut payload_buffer = vec![0; payload_length];
    stream.read_exact(payload_buffer.as_mut_slice()).unwrap();
    let message_type = payload_buffer[0];

    TerrariaMessage {
        kind: TerrariaMessageKind::try_from(message_type).expect("Invalid message type."),
        payload: payload_buffer[1..].to_vec(),
        length: length as usize,
    }
}

struct TerrariaMessage {
    kind: TerrariaMessageKind,
    payload: Vec<u8>,
    length: usize,
}

impl Debug for TerrariaMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let payload_display = match String::from_utf8(self.payload.clone()) {
            Ok(string) => format!("{:?}", string.trim()),
            Err(_) => format!("{:?}", self.payload),
        };

        write!(
            f,
            "kind = {:?}, length = {}, payload = {}",
            self.kind,
            self.length,
            payload_display
        )
    }
}

#[derive(Debug, TryFromPrimitive, PartialEq)]
#[repr(u8)]
enum TerrariaMessageKind {
    ConnectRequest = 1u8,
    PlayerInfo = 4u8,
    ClientUUID = 68u8,
}
