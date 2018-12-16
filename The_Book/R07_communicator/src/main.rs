// R07_communicator
// Learning Rust
// Play with modules
// 2018-10-28	PV

#![allow(dead_code)]

extern crate communicator;

use communicator::network::server;                      // bring namespace in scope
use communicator::network::server::*;                   // bring namespace content in scope
use communicator::network::common::common_connect;      // bring name in scope

fn main() {
    communicator::connect();
    communicator::client::connect();
    communicator::network::connect();
    communicator::network::server::connect();
    communicator::network::common::common_connect();
    communicator::network::helpers::connect();

    println!();
    server::connect();
    connect();              // same as previous line because of use server:;*
    common_connect();       // name has specifically been imported in scope
}
