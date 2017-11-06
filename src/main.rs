extern crate ws;
extern crate json;

use ws::{connect, Handler, Sender, Handshake, Result, Message};

// Here we explicity indicate that the Client needs a Sender,
struct Client {
    out: Sender,
}

fn parse_market_data(mkt_data: json::JsonValue) {
    println!("Got MarketData: {}", mkt_data)
}

fn parse_raw(raw: Message) {
    // msg -> String -> &str -> enum
    let msg = &String::from(raw.as_text().unwrap());
    let parsed_raw = json::parse(&*msg).unwrap();

    // Start of borrow
    {
        let channel = &parsed_raw[0];
        if channel == 1010 {
            println!("Got Heartbeat");
            return;
        }
    }
    // End of borrow

    parse_market_data(parsed_raw);

}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send(r#"{"command":"subscribe","channel":"BTC_ETH"}"#)
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        parse_raw(msg);
        Ok(())
    }
}

fn main() {
    connect("wss://api2.poloniex.com", |out| Client { out: out } ).unwrap()
}
