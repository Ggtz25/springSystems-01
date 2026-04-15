use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} is processing a task", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} is shutting down", id);
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

fn assignment3() {
    let pool = ThreadPool::new(4);

    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }

    println!("Main thread waiting for tasks to complete...");
}

const TERMINATION_SIGNAL: i32 = -1;

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} is exiting", id);
            break;
        }

        println!("Consumer {} consumed {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
}

fn assignment4() {
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = Vec::new();
    let mut consumer_handles = Vec::new();

    let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;

    for id in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        producer_handles.push(thread::spawn(move || {
            producer(id, tx_clone, items_per_producer);
        }));
    }

    for id in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    for handle in producer_handles {
        handle.join().unwrap();
    }

    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn main() {
    println!("Assignment 3:");
    assignment3();

    println!("\nAssignment 4:");
    assignment4();
}