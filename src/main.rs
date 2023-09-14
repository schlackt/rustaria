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
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buffer = String::new();

    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .map(|message| message.into_bytes())
        .take_while(|line| !line.is_empty())
        .collect();

    for message in request {
        let length = message[0] as u16 | ((message[1] as u16) << 8);
        let message_type = message[2];
        println!("--- message received ---");
        dbg!(length);
        dbg!(message_type);
        println!("{}", String::from_utf8(message[3..].to_vec()).unwrap());
    }

    println!("Request: {:#?}", buffer.as_bytes());
}