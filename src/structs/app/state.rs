use std::fmt;
use std::collections::HashMap;
use tui::layout::Rect;
use structs::ui::TopTabs;
use structs::app::{AppMode, CmdCallback, Command};
use reducers::{CommandGen, commands};

pub struct CommandHandler {
    pub cmd_reg: HashMap<String, CommandGen>
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {
            cmd_reg: HashMap::new()
        }
    }

    pub fn default() -> Self {
        let mut handler = CommandHandler::new();
        handler.cmd_reg.insert("exec".to_string(), commands::helloworld);
        handler
    }
}

#[derive(Clone)]
pub struct AppState {
    pub mode: AppMode,
    pub size: Rect,
    pub tabs: TopTabs,
    pub command: String,
    pub console_txt: String,
    pub cmd_reg: HashMap<String, CmdCallback>,
    pub cmd_str_queue: HashMap<String, String>,
    pub cmd_running: Vec<Command>,
    pub cmd_ended: Vec<Command>,
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
            cmd_reg: HashMap::new(),
            cmd_str_queue: HashMap::new(),
            cmd_running: Vec::new(),
            cmd_ended: Vec::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let state = AppState::new();
        // state.cmd_reg.insert("exit".to_string(), Self::exit);
        state
    }
}

impl fmt::Debug for AppState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("AppState")
            .field("mode", &self.mode)
            .field("size", &self.size)
            .field("tabs", &self.tabs)
            .finish()
    }
}
