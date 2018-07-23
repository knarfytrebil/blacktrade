use cpython::{PyDict, PyResult, Python, ToPyObject};
use termion::event;
use tui::layout::Rect;

// use tui::backend::Backend;
// use tui::backend::MouseBackend;
// use tui::Terminal;

use redux::Reducer;
use store::tab::TopTabs;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeCategory {
    Normal,
    Command,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppMode {
    pub category: ModeCategory,
    pub symbol: String,
}

impl AppMode {
    pub fn get_mode(mode_name: &str) -> AppMode {
        match mode_name {
            "normal" => AppMode {
                category: ModeCategory::Normal,
                symbol: String::from("NORM"),
            },
            "command" => AppMode {
                category: ModeCategory::Command,
                symbol: String::from("CTRL"),
            },
            &_ => AppMode {
                category: ModeCategory::Command,
                symbol: String::from("Unknown Mode"),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub mode: AppMode,
    pub size: Rect,
    pub tabs: TopTabs,
    pub command: String,
    pub console_txt: String,
    pub exiting: bool,
}

#[derive(Clone, Debug)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    ConsoleWrite(String),
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            mode: AppMode::get_mode("normal"),
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec![String::from("Console")],
                selection: 0,
            },
            command: String::from(""),
            console_txt: String::from(""),
            exiting: false,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::new()
    }
}

impl Reducer for AppState {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        match action {
            AppAction::ResizeApp(size) => {
                self.size = size;
            }
            AppAction::Keyboard(key_evt) => {
                Self::key_event_handler(self, key_evt);
            }
            AppAction::ConsoleWrite(line) => {
                self.console_txt.push_str(&line);
            }
        }
        Ok(self.clone())
    }
}

// Event Handlers for Key Input
impl AppState {
    fn key_event_handler(&mut self, evt: event::Key) {
        match self.mode.category {
            ModeCategory::Normal => {
                Self::normal_key_handler(self, evt);
            }
            ModeCategory::Command => {
                Self::command_key_handler(self, evt);
            }
        }
    }

    fn normal_key_handler(&mut self, evt: event::Key) {
        match evt {
            event::Key::Char(':') => {
                self.set_mode("command");
            }
            _ => {
                info!("unimplemented");
            }
        }
    }

    fn command_key_handler(&mut self, evt: event::Key) {
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
                if &cmd == "q" {
                    self.exiting = true;
                } else {
                    info!("Command Issued: {:?}", cmd);
                    let line = format!("{{fg=green [command] }} {}\n", &cmd);
                    self.console_txt.push_str(&line);
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    let quotes = vec![
                        Quote {
                            symbol: String::from("btc"),
                            bid_price: 1000,
                            ask_price: 1100,
                            bid_size: 100,
                            ask_size: 100,
                            timestamp: 13213123,
                        },
                        Quote {
                            symbol: String::from("btc"),
                            bid_price: 1000,
                            ask_price: 1100,
                            bid_size: 100,
                            ask_size: 100,
                            timestamp: 13213123,
                        },
                    ];
                    let res = run_python(
                        py,
                        &quotes,
"\n\nimport random\n\ndef trade():\n    if random.randint(0, 2) == 0:\n        return 1    \n    return sum([i['ask_price'] for i in data])\n"
                    );
                    self.console_txt
                        .push_str(&format!("{{fg=yellow [pyout] }}{:?}\n", &res));
                    let res = py.eval("sum([1, 2, 3, 4])", None, None);
                    self.console_txt
                        .push_str(&format!("{{fg=yellow [pyout] }}{:?}\n", &res));
                }
            }
            event::Key::Char(_char) => {
                self.command.push(_char);
            }
            _ => {}
        }
    }

    // helper functions
    fn set_mode(&mut self, mode: &str) {
        match mode {
            "command" => {
                self.mode = AppMode::get_mode("command");
                self.command.push(':');
            }
            "normal" => {
                self.mode = AppMode::get_mode("normal");
                self.command.clear();
            }
            _ => {}
        }
    }
}

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

fn run_python(py: Python, data: &Vec<Quote>, func_code: &str) -> PyResult<(i64)> {
    match py.run(func_code, None, None) {
        Ok(_) => {
            let globals: PyDict = py.eval("globals()", None, None)?.extract(py)?;
            globals.set_item(py, "data", data)?;
            let res = py.eval("trade()", Some(&globals), None)?.extract(py)?;
            return Ok(res);
        }
        Err(e) => {
            return Err(e);
        }
    }
}
