extern crate ws;
extern crate json;

use ws::{ connect, Handler, Sender, Handshake, Result, Message };
use std::collections::HashMap;

// Here we explicity indicate that the Client needs a Sender,
struct Client {
    out: Sender,
}

struct Orderbook {
    version: i32,                                               // Version
    asks: HashMap<String, isize>,                               // Ask Orders
    bids: HashMap<String, isize>,                               // Bid Orders
}

fn parse_market_data(mkt_data: json::JsonValue) {

    // =======================
    // Determin Data Type
    // =======================
    let version = &mkt_data[1];                                 // Version of the Orderbook
    let orderbook_data = &mkt_data[2][0];                       // Orderbook Data
    let orderbook_flag = &orderbook_data[0];                    // Orderbook Type Identifier

    // Process the initial full orderbook
    if orderbook_flag == "i" {
        // Get Orderbook from "i" initial
        let raw_orderbook = &orderbook_data[1]["orderBook"];
        let ask_orders = &raw_orderbook[0];                     // Ask Orders
        let bid_orders = &raw_orderbook[1];                     // Bid Orders
        println!("[{}][FULL]:{}", version, raw_orderbook);  
        println!("[{}][0]:{}", version, ask_orders);
        println!("[{}][1]:{}", version, bid_orders);
    }

    // Process the incremental orderbook
    if orderbook_flag == "o" {
        println!("[{}][INCREMENTAL]:{}", version, orderbook_data);
    }
       
}

fn parse_raw(raw: Message) {

    // msg -> String -> &str -> enum
    let msg = &String::from(raw.as_text().unwrap());
    let parsed_raw = json::parse(&*msg).unwrap();
    
    { // Start of borrow
        let channel = &parsed_raw[0];
        if channel == 1010 {
            println!("[HEARTBEAT]");
            return;
        }
    } // End of borrow
    
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
    // Connect to websocket
    connect("wss://api2.poloniex.com", |out| Client { out: out } ).unwrap()
}
