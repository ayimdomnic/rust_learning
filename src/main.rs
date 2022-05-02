use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

//multiproducer singleconsumer - many channels can send messages to the channel only one can receive

fn main() {
    //two types of threads
    //1. Native thread(n to n)
    //2. Green threads (m to n)
    //Native thread is a thread that is created by the operating system
    //Green thread is a thread that is created by the rust compiler
    //Rust is a multi-threaded language
    //Rust is a concurrent language
    //Concurrency hazards
    //1. Race Conditions
    //2. Deadlocks

    let outer_scope: i32 = 412;

    let join_handle = thread::spawn(move || outer_scope * 2);

    //blocking
    let result = join_handle.join();

    match result {
        Ok(value) => {
            println!("{}", value)
        }
        Err(_) => {}
    }

    let (john_tx, john_rx) = mpsc::channel();
    let (jane_tx, jane_rx) = mpsc::channel();

    let john_handle = thread::spawn(move || john_chat(jane_tx, john_rx));

    let jane_handle = thread::spawn(move || jane_chat(john_tx, jane_rx));

    match john_handle.join() {
        Ok(_) => {}
        Err(_) => {}
    }

    match jane_handle.join() {
        Ok(_) => {}
        Err(_) => {}
    }

    //shared memory
    //messaging
}

fn jane_chat(john_tx: Sender<&str>, jane_rx: Receiver<&str>) {
    let result = jane_rx.recv();
    println!("{}", result.unwrap());

    let _send_result = john_tx.send("Hello, John");
}

fn john_chat(jane_tx: Sender<&str>, john_rx: Receiver<&str>) {
    let _send_result = jane_tx.send("Hello, Jane");
    let result = john_rx.recv();

    println!("{}", result.unwrap());
}
