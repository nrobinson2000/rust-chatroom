use std::io::prelude::*;
use std::net::TcpStream;
use std::io;
use std::io::stdin;
use std::str;
use std::borrow::Cow;


fn main() {
//    println!("Please enter the IP address of the server.");
//
//    let mut input = String::new();
//    stdin().read_line(&mut input).expect("error: unable to read user input");
//
//    println!("{}", input);

    let host = "127.0.0.1";
    let port = "7878";

    let address = host.to_owned() + ":" + port;

    let mut stream = TcpStream::connect(address).unwrap();

    readFromServer(&stream);


    let mut name = String::new();
    stdin().read_line(&mut name).expect("error: unable to read user input");

    sendToServer(&stream, &mut name);

    readFromServer(&stream);

    loop {
        let mut user_message = String::new();
        stdin().read_line(&mut user_message).expect("error: unable to read user input");
        sendToServer(&mut stream, &mut user_message);
        if user_message.trim() == String::from("{quit}") {
            println!("Quitting!");
            break;
        }
    }


//    println!("Hello world!");
}

fn readFromServer(mut stream: &TcpStream) {
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer);

    let mut vecInput = vec![];
    vecInput.extend_from_slice(&buffer);
    let mut input = String::from_utf8(vecInput).unwrap();

    input.retain(|c| c != '\0');

    println!("{}", input);
}


fn sendToServer(mut stream: &TcpStream, mut message: &String) {
    stream.write(message.as_bytes());
}
