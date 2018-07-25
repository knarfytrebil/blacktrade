use cpython::{PyDict, Python, ToPyObject};

pub struct Quote {
    pub symbol: String,
    pub bid_price: i32,
    pub ask_price: i32,
    pub bid_size: i32,
    pub ask_size: i32,
    pub timestamp: i64,
}

impl ToPyObject for Quote {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "symbol", self.symbol.as_str()).unwrap();
        dict.set_item(py, "bid_price", self.bid_price).unwrap();
        dict.set_item(py, "ask_price", self.ask_price).unwrap();
        dict.set_item(py, "bid_size", self.bid_size).unwrap();
        dict.set_item(py, "ask_size", self.ask_size).unwrap();
        dict.set_item(py, "timestamp", self.timestamp).unwrap();
        return dict;
    }
}

pub fn get_quotes() -> Vec<Quote> {
    let quotes = vec![
        Quote {
            symbol: String::from("btc-usd"),
            bid_price: 1000,
            ask_price: 1100,
            bid_size: 100,
            ask_size: 100,
            timestamp: 13213123,
        },
        Quote {
            symbol: String::from("btc-usd"),
            bid_price: 1000,
            ask_price: 1100,
            bid_size: 100,
            ask_size: 100,
            timestamp: 13213123,
        },
    ];

    return quotes;
}
