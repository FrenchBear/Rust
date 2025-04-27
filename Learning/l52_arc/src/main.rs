// arc
// Use of arc to allow sharing references to threads
// Note that Arc smart pointer must own the data (can't call Arc::new on a ref, we'll still have leaking issues in this case)
//
// 2025-04-27   PV

#![allow(unused)]

use std::sync::Arc;
use threadpool::ThreadPool;

fn main() {
    let lst_vec = vec![1, 2, 3, 4, 5, 6];
    let lst = Arc::new(lst_vec); // Wrap the Vec in an Arc
    run_threads(&lst);
}

fn run_threads(lst: &Arc<Vec<i32>>) {
    // lst is now a reference to an Arc
    let pool = ThreadPool::new(4);
    for i in 1..10 {
        let lst_clone = Arc::clone(lst); // Create a new Arc pointer for each thread
        pool.execute(move || {
            // Do something with lst_clone
            let _len = lst_clone.len(); // Example usage
            process(&(*lst_clone));
        });
    }

    pool.join(); // Wait for all threads in the pool to complete
    println!("Done");
}

fn process(lst_clone: &Vec<i32>) {
    // Do something
}
