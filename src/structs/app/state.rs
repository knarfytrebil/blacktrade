use std::collections::HashMap;
use std::fmt;
use structs::app::{Command};
use structs::ui::TopTabs;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

// use structs::app::CmdCallback;

const DATA: &'static str = r#"
{
    "mode": {
        "category": "normal",
        "symbol": "NORM"
    },
    "tabs": {
        "titles": [
            "Console", 
            "tab -2", 
            "tab-3"
        ],
        "selection": 0
    },
    "command": "",
    "console_output_lines": [],
    "console_scroll": 0,
    "cmd_str_queue": {},
    "cmd_running": {},
    "cmd_ended": {}
}
"#;

#[derive(Clone, Serialize, Deserialize)]
pub struct AppState {
    pub json_store: Value,
    pub tabs: TopTabs,
    pub command: String,
    pub console_output_lines: Vec<String>,
    pub cmd_str_queue: HashMap<String, String>,
    pub console_scroll: u16,
    pub cmd_running: Vec<Command>,
    pub cmd_ended: Vec<Command>,
}

impl AppState {
    pub fn new() -> AppState {
        let state: Value = serde_json::from_str(DATA).expect("JSON Error!");

        AppState {
            json_store: state,
            tabs: TopTabs {
                titles: vec![
                    String::from("Console"),
                    String::from("tab - 2"),
                    String::from("tab - 3"),
                ],
                selection: 0,
            },
            command: String::from(""),
            console_output_lines: vec![],
            console_scroll: 0 as u16,
            cmd_str_queue: HashMap::new(),
            cmd_running: Vec::new(),
            cmd_ended: Vec::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::new()
    }
}

impl fmt::Debug for AppState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("AppState")
            .field("tabs", &self.tabs)
            .finish()
    }
}
