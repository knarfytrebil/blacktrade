use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structs::app::Command;

const DATA: &'static str = r#"
{
    "mode": "normal",
    "tabs_titles": ["Console", "Chat", "Settings"],
    "tabs_selection": 0,
    "command": "",
    "console_output_lines": []
}
"#;

#[derive(Clone, Serialize, Deserialize)]
pub struct AppState {
    pub json_store: Value,
    pub cmd_str_queue: HashMap<String, String>,
    pub cmd_running: Vec<Command>,
    pub cmd_ended: Vec<Command>,
}

impl AppState {
    pub fn new() -> AppState {
        let state: Value = serde_json::from_str(DATA)
            .expect("JSON Error!");

        AppState {
            json_store: state,
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
            .field("tabs", &self.json_store)
            .finish()
    }
}
