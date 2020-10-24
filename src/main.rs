//! Ping client

use std::thread;
use std::time::Duration;

fn main() {
    println!("Connecting to Pong server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..51 {
        println!("Sending Ping {}...", request_nbr);
        requester.send("Ping", 0).unwrap();
        thread::sleep(Duration::from_millis(50));
        requester.recv(&mut msg, 0).unwrap();
        println!("Received Pong {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}
