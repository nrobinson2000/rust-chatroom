use std::io::prelude::*;
use std::net::TcpStream;
use std::io;
use std::io::stdin;
use std::str;
use std::borrow::Cow;
use chatroom::ThreadPool;


fn main() {
//    println!("Please enter the IP address of the server.");
//
//    let mut input = String::new();
//    stdin().read_line(&mut input).expect("error: unable to read user input");
//
//    println!("{}", input);

    // Client needs to be multi threaded too

    // One thread for sending, one thread for receiving

    // One worker for sending
    // One worker for receiving
//    let pool = ThreadPool::new(2);
//
//    pool.execute(|| {
//
//
//
//        //handle_connection(stream);
//
//
//
//    });


    // Create socket connection
    let host = "127.0.0.1";
    let port = "7878";
    let address = host.to_owned() + ":" + port;
    let mut stream = TcpStream::connect(address).unwrap();

    // Get welcome message from server
    readFromServer(&stream);

    // Get the username
    let mut name = String::new();
    stdin().read_line(&mut name).expect("error: unable to read user input");

    // Send the username to the server
    sendToServer(&stream, &mut name);

    // Get hello message from the server
    readFromServer(&stream);

    // Reserve two threads for reading and writing
    let pool = ThreadPool::new(2);

//    let read_stream = stream.try_clone().unwrap();
//
//    // Spawn reading thread
//    pool.execute(move || {
//        read_handler(&read_stream);
//    });

    let write_stream = stream.try_clone().unwrap();

    // Spawn writing thread
    pool.execute(move || {
        write_handler(&write_stream);
    });


//    loop {
//        let mut user_message = String::new();
//        stdin().read_line(&mut user_message).expect("error: unable to read user input");
//        sendToServer(&mut stream, &mut user_message);
//        if user_message.trim() == String::from("{quit}") {
//            println!("Quitting!");
//            break;
//        }
//    }


//    println!("Hello world!");
}

fn read_handler(mut stream: &TcpStream) {
    loop {
        readFromServer(&stream);
    }
}

// Get messages from the user and write to the stream
fn write_handler(mut stream: &TcpStream) {
    // DEBUG
    //println!("Test!");
    loop {
        let mut user_message = String::new();
        stdin().read_line(&mut user_message).expect("error: unable to read user input");
        sendToServer(&mut stream, &mut user_message);
        if user_message.trim() == String::from("{quit}") {
            println!("Quitting!");
            break;
        }
    }
}


fn readFromServer(mut stream: &TcpStream) -> String {
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer);

    let mut vecInput = vec![];
    vecInput.extend_from_slice(&buffer);
    let mut input = String::from_utf8(vecInput).unwrap();

    input.retain(|c| c != '\0');

    println!("{}", input);

    input
}


fn sendToServer(mut stream: &TcpStream, mut message: &String) {
    stream.write(message.as_bytes());
}
