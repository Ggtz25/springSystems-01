use std::sync::{Arc, Mutex};
use std::thread;

fn assignment1() {
    let mut handles = Vec::new();

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads completed.");
}

fn assignment2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut value = counter.lock().unwrap();
                *value += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

fn main() {
    println!("Assignment 1:");
    assignment1();

    println!();

    println!("Assignment 2:");
    assignment2();
}