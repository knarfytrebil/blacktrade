// A WebSocket client that sends one message then closes
extern crate ws;

use ws::{connect, CloseCode};

fn main() {
    connect("wss://api2.poloniex.com", |out| {
        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()
} 
