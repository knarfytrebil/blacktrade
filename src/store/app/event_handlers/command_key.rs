use cpython::{PyDict, PyResult, Python, ToPyObject};
// use std::fmt;
use store::app::AppState;
use termion::event;

// impl fmt::Display for PyResult<i64> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "{:?}", self)
//     }
// }

struct Quote {
    symbol: String,
    bid_price: i32,
    ask_price: i32,
    bid_size: i32,
    ask_size: i32,
    timestamp: i64,
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

fn run_python(py: Python, data: &Vec<Quote>, func_code: &str) -> PyResult<i64> {
    match py.run(func_code, None, None) {
        Ok(_) => {
            let globals: PyDict = py.eval("globals()", None, None)?.extract(py)?;
            globals.set_item(py, "data", data)?;
            let res = py.eval("main()", Some(&globals), None)?.extract(py)?;
            return Ok(res);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn get_quotes() -> Vec<Quote> {
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

fn get_snippet() -> &'static str {
    return "\n\nimport random\n\ndef main():\n    if random.randint(0, 2) == 0:\n        return 1    \n    return sum([i['ask_price'] for i in data])\n";
}

macro_rules! format_output {
    ($color:expr, $title:expr, $output:expr) => {
        format!("{{fg={} [{}] }} {:?}\n", $color, $title, $output)
    };
}

impl AppState {
    pub fn command_key_handler(&mut self, evt: event::Key) {
        match evt {
            event::Key::Esc => {
                self.set_mode("normal");
            }
            event::Key::Backspace => {
                if self.command == ":" {
                    self.set_mode("normal");
                } else {
                    self.command.pop();
                }
            }
            // Must be above Char(_char)
            event::Key::Char('\n') => {
                let cmd = self.command.split_off(1);
                match cmd.as_str() {
                    "q" => {
                        self.exiting = true;
                    }
                    _ => {
                        info!("Command Issued: {:?}", cmd);
                        let line = format_output!("green", "command", &cmd);
                        self.console_txt.push_str(&line);
                        let gil = Python::acquire_gil();
                        let py = gil.python();
                        let quotes = get_quotes();
                        let code = get_snippet();
                        let res = run_python(py, &quotes, code);
                        self.console_txt
                            .push_str(&format_output!("yellow", "pyout", &res));
                    }
                }
            }
            event::Key::Char(_char) => {
                self.command.push(_char);
            }
            _ => {}
        }
    }
}
