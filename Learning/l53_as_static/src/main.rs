// as_static
// How to force a reference to be 'static to disable issues with references leaking in a thread
//
// 2025_04_27   PV

#![allow(unused)]

use threadpool::ThreadPool;

fn main() {
    let lst = Vec::<i32>::new();
    run_threads(as_static(&lst));
}

fn as_static<'a, T>(data: &'a T) -> &'static T {
    unsafe {
        std::mem::transmute::<&'a T, &'static T>(data)
    }
}

// Note that I can't remplace &'static Vec<i32> by &'static [i32] as Clippy suggests, not sure why...
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
