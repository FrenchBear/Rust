// R07_communicator
// Learning Rust
// Play with modules
// 2018-10-28	PV


pub fn connect() {
    println!("communicator::network::connect()");
}

pub mod server;

pub mod helpers;

pub mod common {
    pub fn common_connect() {
        println!("communicator::network::common::connect()");
    }
}
