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
    asks: HashMap<isize, isize>,                                // Ask Orders
    bids: HashMap<isize, isize>,                                // Bid Orders
}

// fn init_orderbook(entries: json::object::Iter) -> HashMap<f32, f32> {
//     let mut _orderbook = HashMap::<f32, f32>::new();
//     return _orderbook;
// }

fn parse_market_data(mkt_data: json::JsonValue) {

    // ######################## 
    // # Determin Data Type
    // ########################
    let version = &mkt_data[1];                                 // Version of the Orderbook
    let orderbook_data = &mkt_data[2][0];                       // Orderbook Data
    let orderbook_flag = &orderbook_data[0];                    // Orderbook Type Identifier

    // Process the initial full orderbook
    // Get Orderbook from "i" initial
    if orderbook_flag == "i" {
        let raw_orderbook = &orderbook_data[1]["orderBook"];
        let mut ask_orders = raw_orderbook[0].entries();        // Ask Orders
        let mut bid_orders = raw_orderbook[1].entries();        // Bid Orders
        // println!("[AKS COUNT][0]:{}", ask_orders.count());
        // println!("[BID COUNT][1]:{}", bid_orders.count());
        for x in bid_orders {
            println!("[ENTRY]:{:?}", x);
        }

        // ask_orders.map(|x| println!("{:?}", x));
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
