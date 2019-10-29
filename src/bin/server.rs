// Lab 4: Chatroom
// Author: Nathan Robinson

// Reference:
// https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
// Use:
// Inspiration for multi-threaded server

extern crate chatroom;

use chatroom::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::BufReader;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(20);

    println!("Waiting for connection...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let welcome = "Welcome to the NP chatroom! Please type your name and press enter!";
    stream.write(welcome.as_bytes());


    let username = get_client_message(&stream);

    println!("{} has joined the chat!", username);

    let hello_message = "Hello ".to_owned() + &username + "! If you ever want to quit, type {quit} to exit.";
    stream.write(hello_message.as_bytes());

    loop {


        let message = get_client_message(&stream);


        if message == String::from("{quit}") {
            println!("{} has left the chat!", username);
            break;
        }


        println!("{}: {}", username, message);
    }


    stream.flush().unwrap();
}

fn get_client_message(mut stream: &TcpStream) -> String {
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer);
    to_clean_string(&mut buffer)
}

fn to_clean_string(buffer: &mut [u8]) -> String {
    // Lovely String stuff
    let mut vecInput = vec![];
    vecInput.extend_from_slice(&buffer);
    let mut input = String::from_utf8(vecInput).unwrap();
    input.retain(|c| c != '\0');
    trim_newline(&mut input);
    input
}

fn trim_newline(s: &mut String) {
    s.pop();
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
