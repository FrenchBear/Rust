// l54_leak
// Lock a reference (prevent memory to be deallocated, effectively making its lifetime 'static) to disable issues with references leaking in a thread
//
// 2025_04_27   PV

#![allow(unused)]

use threadpool::ThreadPool;

fn main() {
    let lst = Vec::<i32>::new();
    let lst_box = Box::new(lst);
    let lst_static: &'static Vec<i32> = Box::leak(lst_box);
    let res = run_threads(lst_static);
}

fn run_threads(lst: &'static Vec<i32>) {
	let pool = ThreadPool::new(4);
	for i in 1..10 {
		pool.execute(move || {
			// Do something with lst
            let l = lst.len();
			}
		);
	}
	
	// Wait for all threads to terminate
	println!("Done");
}
