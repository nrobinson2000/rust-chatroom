// Lab 4: Chatroom
// Author: Nathan Robinson
// Chat Server

// Reference:
// https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
// Use:
// Inspiration for ThreadPool

// Chatroom library imports (lib.rs)
extern crate chatroom;

use chatroom::ChatMessage;
use chatroom::UserStream;
use chatroom::ThreadPool;

// Standard Library imports
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::borrow::Borrow;

// Maximum number clients allowed to connect to the server
static MAXIMUM_CLIENTS: usize = 20;

fn main() {
    // Create server socket
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Reserve threads
    let pool = ThreadPool::new(MAXIMUM_CLIENTS + 1);

    println!("Waiting for connection...");

    // Create Vector of client sockets
    let mut clients = Vec::new();

    // Channel for communication
    let (tx, rx) = mpsc::channel();

    // Dedicated thread for sending to all clients
    pool.execute(move || {
        for recv in rx {
            handle_outgoing_messages(&clients, &recv);
        }
    });

    // Process clients in dedicated threads when they connect
    for stream in listener.incoming() {

        // Create reference to client socket
        let mut stream = stream.unwrap();

        // Create clone of the MPSC transmitter
        let tx1 = mpsc::Sender::clone(&tx);

        // TEST Try to make a reference to clients (not working)
        // let mut tempClients = clients.clone();

        // TEST Try to add the client (not working)
        // let mut stream_copy = stream.try_clone().unwrap();
        // add_client(&mut clients,stream_copy,String::from("test"));

        // Spawn the listener thread
        pool.execute(move || {

            // Allow client to login and get username
            let username = handle_sign_in(&mut stream);

            let mut temp_stream = stream.try_clone().unwrap();
            let mut temp_username = username.clone();

            // Push client object into vector (not working)
            //add_client(&mut clients,tempStream,tempUsername);

            // Clone the client socket and the client username
            let mut incoming_stream = stream.try_clone().unwrap();
            let incoming_username = username.clone();

            handle_incoming_messages(&mut incoming_stream, incoming_username, tx1);
        });

        // EXPERIMENTAL CODE
//        let outgoing_stream = stream.try_clone().unwrap();
//        let outgoing_messages = messages.clone();
//        let outgoing_username = username.clone();
//        pool.execute(move || {
//           handle_outgoing_messages(&outgoing_stream, outgoing_messages, outgoing_username);
//        });
//
//        let rx1 = rx.borrow();
//
//        for try_recv in rx1 {
//            handle_outgoing_messages(clients.borrow(), &recv);
//        }
    }

    println!("Shutting down.");
}

fn add_client(clients: &mut Vec<UserStream>, stream: TcpStream, username: String) {
    clients.push(UserStream::new(stream.try_clone().unwrap(), username));
}

fn handle_sign_in(mut stream: &mut TcpStream) -> String {
    // Send welcome message to client
    let welcome = "Welcome to the NP chatroom! Please type your name and press enter!";
    send_to_client(&mut stream, String::from(welcome));

    // Get username from client
    let username = get_client_message(&mut stream);

    println!("{} has joined the chat!", username);

    let hello_message = "Hello ".to_owned() + &username + "! If you ever want to quit, type {quit} to exit.";
    stream.write(hello_message.as_bytes());

    return username;
}

// Process sending messages to clients
fn handle_outgoing_messages(clients: &[UserStream], mut message: &ChatMessage) {
    println!("Sending message to all connected clients...");

    // This loop never iterates because clients is never filled
    for client in clients {

        // Get reference to client socket
        let mut stream = client.getStream();

        // DEBUG (not working)
        // println!("Sending to {}", client.getUsername().borrow());

        // TEST
        send_to_client(&stream, String::from("TEST"));

        // Try to get username of client (not working)
        // let username = client.getUsername();
        // Dummy
        let username = String::new();

        // Get username of message, and contents of message
        let message_username = message.clone().getUsername();
        let message_contents = message.clone().getMessage();

        // Only send packet to clients other than the originating user client
        if String::from(username.clone()) != message_username {

            // Create message packet to send to client
            let mut sent_message = String::new();
            sent_message.push_str(message_username.borrow());
            sent_message.push_str(message_username.borrow());

            // DEBUG
            println!("Sending {} to {}", sent_message, username);

            send_to_client(&stream, sent_message);
        }
    }
    println!("Finished sending.");
}

// Send packet to client
fn send_to_client(mut stream: &TcpStream, message: String) {
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
        let output_message = ChatMessage::new(username.clone(), message);
        tx.send(output_message).unwrap();
    }

    stream.flush().unwrap();
}

fn get_client_message(stream: &mut TcpStream) -> String {
    let mut buffer = [0u8; 1024];
    // If the client force closes the connection
    if stream.read(&mut buffer).unwrap() == 0 {
        return String::from("ERROR: CLIENT DISCONNECTED");
    }
    to_clean_string(&mut buffer)
}

fn to_clean_string(buffer: &mut [u8]) -> String {
    let mut vec_input = vec![];
    vec_input.extend_from_slice(&buffer);
    let mut input = String::from_utf8(vec_input).unwrap();
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
