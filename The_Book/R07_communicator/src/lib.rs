// R07_communicator
// Learning Rust
// Play with modules
// 2018-10-28	PV

//#![allow(dead_code)]

pub mod client;

pub mod network;


pub fn connect() {
    println!("communicator::connect()");
}


#[cfg(test)]
mod tests {
    //use super::client;                // best solution to import client namesace in tests module

    #[test]
    fn it_works() {
        //client::connect();            // we're in a separate module, this doesn't work
        ::client::connect();            // start from crate root
        super::client::connect();       // super moves one level up in the hierarchy of modules
    }
}
