// Lab 4: Chatroom
// Author: Nathan Robinson

// Reference:
// https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
// Use:
// Inspiration for multi-threaded server

extern crate chatroom;

use chatroom::ChatQueue;
use chatroom::ChatMessage;
use chatroom::UserStream;

use chatroom::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::BufReader;
use std::str;
use std::sync::mpsc;
use std::borrow::Borrow;


fn main() {

    // Create server socket
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//    let buf_listener = BufReader::new(listener);

    // Reserve threads
    let pool = ThreadPool::new(20);

    println!("Waiting for connection...");

//    let messages = ChatQueue::new();

//    let mut bufClients = Vec::new();

    let mut clients = Vec::new();

    // Channel for communication
    let (tx, rx) = mpsc::channel();


    // Dedicated thread for sending to all clients
    pool.execute(move || {

//        let rx1 = rx.borrow();





        for recv in rx {
            handle_outgoing_messages(&clients, &recv);
        }
    });

    // Process clients in dedicated threads when they connect
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

//        let mut bufStream = BufTcpStream::new(stream).unwrap();


        let tx1 = mpsc::Sender::clone(&tx);

//        let mut tempClients = clients.clone();

        pool.execute(move || {

            // Allow client to login and get username
            let username = handle_sign_in(&mut stream);

            let mut tempStream = stream.try_clone().unwrap();
            let mut tempUsername = username.clone();

//            let mut tempClients = &clients;
//            tempClients.push(UserStream::new(tempStream, tempUsername));
           // clients.push(UserStream::new(tempStream, tempUsername));

            let mut incoming_stream = stream.try_clone().unwrap();

            let incoming_username = username.clone();


            handle_incoming_messages(&mut incoming_stream, incoming_username, tx1);
        });

//        let outgoing_stream = stream.try_clone().unwrap();
//        let outgoing_messages = messages.clone();
//        let outgoing_username = username.clone();
//        pool.execute(move || {
//           handle_outgoing_messages(&outgoing_stream, outgoing_messages, outgoing_username);
//        });

//        let rx1 = rx.borrow();
//
//        for try_recv in rx1 {
//            handle_outgoing_messages(clients.borrow(), &recv);
//        }
    }

    println!("Shutting down.");
}

fn handle_sign_in(mut stream: &mut TcpStream) -> String {
    // Send welcome message to client
    let welcome = "Welcome to the NP chatroom! Please type your name and press enter!";
    sendToClient(&mut stream, String::from(welcome));

    // Get username from client
    let username = get_client_message(&mut stream);

    println!("{} has joined the chat!", username);

    let hello_message = "Hello ".to_owned() + &username + "! If you ever want to quit, type {quit} to exit.";
    stream.write(hello_message.as_bytes());

    return username;
}

// Process sending messages to clients
fn handle_outgoing_messages(mut clients: &[UserStream], mut message: &ChatMessage) {
    for client in clients {
//            let client = client.borrow();

        println!("WTF");

        let mut stream = client.getStream();

        sendToClient(&stream, String::from("Close!"));
//
//            let username = client.getUsername().borrow();
//
//            let message_username = message.getUsername();
//
//            let message_contents = message.getMessage();

//            if String::from(username) != message_username {
////               let sent_message = message_username + ": " + message_contents.borrow();
//
//                let mut sent_message = String::new();
////                sent_message.push_str(message_username);
////                sent_message.push_str(message_username);
//
//                sendToClient(&stream, sent_message);
//            }
    }
}

fn sendToClient(mut stream: &TcpStream, message: String) {
    stream.write(message.as_bytes());
}

// Process receiving messages from clients
fn handle_incoming_messages(mut stream: &mut TcpStream, username: String, tx: mpsc::Sender<ChatMessage>) {
    loop {
        // Get a message from the client
        let message = get_client_message(&mut stream);

        // Client disconnected
        if message == String::from("ERROR: CLIENT DISCONNECTED") {
            println!("{} has left the chat!", username);
            break;
        }

        // Client sent an empty message
        if message.len() == 0 {
            println!("{} sent an empty message", username);
            break;
        }

        // Client sent quit message
        if message == String::from("{quit}") {
            println!("{} has left the chat!", username);
            break;
        }

        // Print the message from the client
        println!("{}: {}", username, message);

        // Send the message to MPSC

        let outputMessage = ChatMessage::new(username.clone(), message);

        tx.send(outputMessage).unwrap();
    }

    stream.flush().unwrap();
}

fn get_client_message(stream: &mut TcpStream) -> String {
    let mut buffer = [0u8; 1024];

    if stream.read(&mut buffer).unwrap() == 0 {
        // Error handling
        return String::from("ERROR: CLIENT DISCONNECTED");
    }
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
