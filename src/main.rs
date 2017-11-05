extern crate ws;
extern crate json;

// use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
use ws::{connect, Handler, Sender, Handshake, Result, Message};

// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
}

fn parse_data(raw: Message) {
    // msg -> String -> &str -> enum
    let msg = &String::from(raw.as_text().unwrap());
    let parsed = json::parse(&*msg).unwrap();
    println!("Got message: {}", parsed);
}

impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send(r#"{"command":"subscribe","channel":"BTC_ETH"}"#)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        parse_data(msg);
        Ok(())
        // self.out.close(CloseCode::Normal)
    }

}

fn main() {
    connect("wss://api2.poloniex.com", |out| Client { out: out } ).unwrap()
}
