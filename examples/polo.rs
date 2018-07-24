extern crate json;
extern crate tui;
extern crate ws;

use std::collections::HashMap;
use tui::Terminal;
use ws::{connect, Handler, Handshake, Message, Result, Sender};

// Here we explicity indicate that the Client needs a Sender,
struct Client {
    out: Sender,
}

struct Orderbook {
    version: i32,               // Version
    asks: HashMap<String, f32>, // Ask Orders
    bids: HashMap<String, f32>, // Bid Orders
                                //    trades: HashMap<String, String>,                              // Trades
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

// fn parse_market_data(mkt_data: json::JsonValue) -> Orderbook {
fn parse_market_data(mkt_data: json::JsonValue) {
    // ########################
    // # Determin Data Type
    // ########################
    let version = mkt_data[1].to_string().parse::<i32>().unwrap(); // [Version] of the Orderbook
    let orderbook_data = &mkt_data[2]; // Orderbook Data
    let orderbook_flag = &orderbook_data[0][0]; // Orderbook Type Identifier

    // ########################
    // # Process the initial full orderbook
    // # Get Orderbook from "i" initial
    // ########################
    if orderbook_flag == "i" {
        let raw_orderbook = &orderbook_data[0][1]["orderBook"];
        let ask_orders = raw_orderbook[0].entries(); // [Ask Orders]
        let bid_orders = raw_orderbook[1].entries(); // [Bid Orders]
        let _orderbook = Orderbook {
            version: version,
            bids: get_orderbook_from_iter(bid_orders),
            asks: get_orderbook_from_iter(ask_orders),
        };
        // return _orderbook;
    }

    // ########################
    // # Process the incremental orderbook and trades
    // # "o" or "t"
    // ########################
    if orderbook_flag != "i" {
        println!("[{}][INCREMENTAL]:{}", version, orderbook_data);
    }
}

fn parse_raw(raw: Message) {
    // msg -> String -> &str -> enum
    let msg = &String::from(raw.as_text().unwrap());
    let parsed_raw = json::parse(&*msg).unwrap();
    {
        // Start of borrow
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
        self.out
            .send(r#"{"command":"subscribe","channel":"BTC_ETH"}"#)
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        parse_raw(msg);
        Ok(())
    }
}

fn main() {
    // let backend = RustboxBackend::new().unwrap();
    // let mut terminal = Terminal::new(backend);
    connect("wss://api2.poloniex.com", |out| Client { out: out }).unwrap()
}
