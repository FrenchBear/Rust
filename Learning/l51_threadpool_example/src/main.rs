// threadpool_example
// Simple example of threadpool
//
// 2025-04-27   PV

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let pool = ThreadPool::new(4); // max 4 threads
    // Use std::thread::available_parallelism().unwrap().get() to get man number of threads
    
    let (tx, rx) = channel();

    for i in 0..8 {
        let tx = tx.clone();
        pool.execute(move || {
            println!("Task {} started", i);
            std::thread::sleep(Duration::from_secs(2));
            println!("Task {} done", i);
            tx.send(i).expect("send failed");
        });
    }

    let mut res:Vec<i32> = Vec::new();
    for _ in 0..8 {
        res.push(rx.recv().expect("recv failed"));
    }

    println!("All tasks completed.");
    println!("Res: {:?}", res);
}