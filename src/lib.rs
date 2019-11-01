// Lab 4: Chatroom
// Author: Nathan Robinson
// Chat Library

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use std::collections::VecDeque;
use std::net::TcpStream;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // DEBUG
        //println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        // DEBUG
        //println!("Shutting down all workers.");

        for worker in &mut self.workers {

            // DEBUG
            //println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
    Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        // println!("Worker {} got a job; executing.", id);

                        job.call_box();


                        //println!("Worker {} completed job", id);
                    }
                    Message::Terminate => {
                        // DEBUG
                        // println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

// Struct and implementation of ChatMessage
#[derive(Clone)]
pub struct ChatMessage {
    username: String,
    message: String,
}

impl ChatMessage {
    // Constructor
    pub fn new(username: String, message: String) -> ChatMessage {
        ChatMessage { username, message }
    }

    pub fn getUsername(self) -> String {
        self.username
    }
    pub fn getMessage(self) -> String {
        self.message
    }
}

// Use a queue to hold ChatMessages (for extra credit)
pub struct ChatQueue {
    queue: VecDeque<ChatMessage>
}

impl ChatQueue {
    pub fn new() -> ChatQueue {
        ChatQueue { queue: VecDeque::new() }
    }

    pub fn enqueue(&mut self, message: ChatMessage) {
        self.queue.push_back(message);
    }

    pub fn dequeue(&mut self) -> ChatMessage {
        self.queue.pop_front().unwrap()
    }

    pub fn getFront(&mut self) -> &ChatMessage {
        self.queue.front().unwrap()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn isEmpty(&mut self) -> bool {
        self.queue.is_empty()
    }
}

impl Clone for ChatQueue
    where VecDeque<ChatMessage>: Clone {
    fn clone(&self) -> Self {
        ChatQueue { queue: self.queue.clone() }
    }
}

// Stream with username
//#[derive (Clone)]
pub struct UserStream {
    stream: TcpStream,
    username: String,
}

impl UserStream {
    pub fn new(stream: TcpStream, username: String) -> UserStream {
        UserStream { stream, username }
    }

    pub fn getStream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn getUsername(self) -> String {
        self.username
    }
}