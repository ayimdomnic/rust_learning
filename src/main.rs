use std::thread;

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

    let join_handle = thread::spawn( move || {
        outer_scope * 2
    });

    //blocking
    let result = join_handle.join();

    match result {
        Ok(value) => {println!("{}", value)},
        Err(_) => {},
    }
}