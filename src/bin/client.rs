// Lab 4: Chatroom
// Author: Nathan Robinson
// Chat Client

use chatroom::ThreadPool;
use std::io::prelude::*;
use std::io::stdin;
use std::net::TcpStream;
use std::process::exit;

fn main() {
    // Create socket connection
    let host = "127.0.0.1";
    let port = "7878";
    let address = host.to_owned() + ":" + port;
    let stream = TcpStream::connect(address).unwrap();

    // Get welcome message from server
    read_from_server(&stream);

    // Get the username
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("error: unable to read user input");

    // Send the username to the server
    send_to_server(&stream, &mut name);

    // Get hello message from the server
    read_from_server(&stream);

    // Reserve two threads for reading and writing
    let pool = ThreadPool::new(2);

    let write_stream = stream.try_clone().unwrap();

    // Spawn writing thread
    pool.execute(move || {
        write_handler(&write_stream);
    });

    let read_stream = stream.try_clone().unwrap();

    // Spawn reading thread
    pool.execute(move || {
        read_handler(&read_stream);
    });
}

fn read_handler(mut stream: &TcpStream) {
    loop {
        read_from_server(&stream);
    }
}

// Get messages from the user and write to the stream
fn write_handler(mut stream: &TcpStream) {
    // DEBUG
    //println!("Test!");
    loop {
        let mut user_message = String::new();
        stdin()
            .read_line(&mut user_message)
            .expect("error: unable to read user input");
        send_to_server(&mut stream, &mut user_message);
        if user_message.trim() == String::from("{quit}") {
            println!("Quitting!");
            exit(0);
        }
    }
}

fn read_from_server(mut stream: &TcpStream) -> String {
    let mut buffer = [0u8; 1024];

    if stream.read(&mut buffer).unwrap() == 0 {
        println!("Server stopped!");
        exit(1);
    }

    let mut vecInput = vec![];
    vecInput.extend_from_slice(&buffer);
    let mut input = String::from_utf8(vecInput).unwrap();

    input.retain(|c| c != '\0');

    println!("{}", input);

    input
}

fn send_to_server(mut stream: &TcpStream, message: &String) {
    stream.write(message.as_bytes()).unwrap();
}
