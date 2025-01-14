// r25_threads
// Learning rust 2024, Threading
//
// 2025-01-13   PV

#![allow(dead_code)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration; // mspc: multiple producer, single consumer

fn main() {
    threads();
    mutexes();
}

fn threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    // Wait for thread to terminate
    handle.join().unwrap();
    println!();

    // Moving data ownership to closure/thread
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
    println!();

    // Channels
    let (tx, rx) = mpsc::channel();

    // Moving tx to a spawned thread and sending “hi”
    thread::spawn(move || {
        let val = String::from("hi from thread");
        tx.send(val).unwrap(); // unwrap() will cause code to panic in case of an error (just for simple learning code)
                               // At this point, val is not available anymore, ownership has been transferred to the receiver.
    });

    // We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel.
    // Once a value is sent, recv will return it in a Result<T, E>.
    // Note: Contrary to rcv(), the try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value
    // holding a message if one is available and an Err value if there aren’t any messages this time.
    let received = rx.recv().unwrap();
    println!("Got: {received}\n");

    // More than 1 message, "seeing" channel in action
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("from"), String::from("the"), String::from("thread")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    println!("Done; channel will not receive more data\n");

    // Multiple producers cloning the transmitter
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // Clone of tx for 1st thread
    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("from"), String::from("the"), String::from("thread")];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("more"), String::from("messages"), String::from("for"), String::from("you")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    println!("Done; channel will not receive more data\n");
}

use std::sync::{Arc, Mutex};


fn mutexes() {
    // Simple example in single-threaded context
    let m = Mutex::new(5);
    // To access data inside the mutex, we have to acquire a lock
    {
        // Lock fails if mutext is already locked (returns a Result<MutexGuard, _>), unwrap panics in this case
        // Note that similar to RefCell<T>, Mutex<T> provides interior mutability
        let mut num = m.lock().unwrap();
        println!("m = {m:?}");
        *num = 6;
    }   // Lock is released when MutexGuard is dropped here
    println!("m = {m:?}");
    println!();
    

    // Using Arc<T> to share ownership across multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    println!();
    
}
