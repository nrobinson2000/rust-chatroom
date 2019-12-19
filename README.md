# rust-chatroom
A client-server chatroom developed using the Rust standard library

## Introduction

For one of my lab assignments in my Network Programming class I decided to implement a chatroom client-server
application using Rust instead of Python or Java because it posed an interesting challenge.

## Usage

In order to compile and run the `client` and `server` applications you must have Rust and Cargo installed.

You can use the [rustup installation tool](https://rustup.rs/) to install Rust.

To test the repository you can do:

```bash
git clone https://github.com/nrobinson2000/rust-chatroom
cd rust-chatroom
```

To run the server application you can do:
```bash
cargo run --bin server
```

To run the client application you can do:
```bash
cargo run --bin client
```

## Update

I have fixed my implementation of the chatroom. The **Known Issues** and **Conclusion** can be disregarded.

In order to fix my chatroom I found that I could use an additional Multi-producer, single-consumer channel to allow
incoming client threads to send `UserStream` objects to the main outgoing thread so that the main thread may store and iterate through the
streams in the `clients` vector.

Now, when multiple clients are connected, every message sent from a client is relayed by the server to all clients but
the client that originally sent the message.

I am glad that I could get this to work. Using Rust to implement this lab was certainly more challenging than
implementing it with Python. I'd like to develop more with Rust in the future.


## Known Issues
While the server can accept client connections and messages it is currently unable to relay messages
to clients.

When spawning each client listener thread any attempts to push a reference to the client onto the
`clients` vector will fail to compile.

On line 75 of server.rs, uncommenting the line to:
```
add_client(&mut clients,temp_stream,temp_username);
```

Will result in the following error:
```
error[E0382]: use of moved value: `clients`
```

I believe that the cause of this issue is because the `Clone` trait is not implemented for
the `clients` vector. If I attempt to use `#[derive (Clone)]` on line 181 in lib.rs I get the following
error:
```
error[E0277]: the trait bound `std::net::TcpStream: std::clone::Clone` is not satisfied
   --> src/lib.rs:183:5
    |
183 |     stream: TcpStream,
    |     ^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `std::net::TcpStream`
    |
    = note: required by `std::clone::Clone::clone`
```

Since the `Clone` trait is not implemented for `TcpStream` the encapsulating type `UserStream`, and the
`UserStream` vector `clients` cannot be cloned.

## Conclusion

I have implemented most of the client-server infrastructure, and I think the only thing preventing
the application from fully functioning is that the client-listener threads are unable to push
`UserStream` references onto the `clients` vector. Because of this, the `handle_outgoing_messages`
function cannot iterate over the `clients` vector since it is empty and therefore cannot send messages
back to clients.
