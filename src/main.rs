extern crate ws;
extern crate serde_json;

// use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
use ws::{connect, Handler, Sender, Handshake, Result, Message};
use serde_json::{Value, Error};

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send(r#"{"command":"subscribe","channel":"BTC_ETH"}"#)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // msg as string type
        let msg = &String::from(msg.as_text()?);
        let msg_slice: &str = &*msg;
        // let v: Value = serde_json::from_str(msg_slice)?;
        println!("Got message: {}", msg_slice);
        // self.out.close(CloseCode::Normal)
        Ok(())
    }

}

fn main() {
    // connect("wss://api2.poloniex.com", |out| Client { out: out } ).unwrap()
    connect("wss://sandboxsocket.b2c2.net/quotes", |out| Client { out: out } ).unwrap()
}
