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

## Conclusion

In order to fix my chatroom I found that I could use an additional Multi-producer, single-consumer channel to allow
incoming client threads to send `UserStream` objects to the main outgoing thread so that the main thread may store and iterate through the
streams in the `clients` vector.

Now, when multiple clients are connected, every message sent from a client is relayed by the server to all clients but
the client that originally sent the message.

I am glad that I could get this to work. Using Rust to implement this lab was certainly more challenging than
implementing it with Python. I'd like to develop more with Rust in the future.