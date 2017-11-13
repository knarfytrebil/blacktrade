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
    asks: HashMap<String, f32>,                                 // Ask Orders
    bids: HashMap<String, f32>,                                 // Bid Orders
}

fn get_orderbook_from_iter(entries: json::object::Iter) -> HashMap<String, f32> {
    let mut _orderbook = HashMap::<String, f32>::new();
    for item in entries {
        let quantity = item.1.to_string().parse::<f32>().unwrap();
        let price = item.0.to_string();
        _orderbook.insert(price, quantity);
    }
    return _orderbook;
}

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
        
        get_orderbook_from_iter(bid_orders);
        get_orderbook_from_iter(ask_orders);

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
