// A WebSocket client that sends one message then closes
extern crate ws;
extern crate env_logger;

use ws::{connect, CloseCode};

fn main() {
    // Setup logging
    env_logger::init().unwrap();

    if let Err(error) = connect("wss://api2.poloniex.com", |out| {
        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to: {:?}", error);
    }
} 
