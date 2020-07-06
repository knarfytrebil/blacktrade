use std::collections::HashMap;
use std::fmt;
use structs::app::{AppMode, Command};
use structs::ui::TopTabs;
use tui::layout::Rect;
// use structs::app::CmdCallback;

#[derive(Clone)]
pub struct AppState {
    pub mode: AppMode,
    pub size: Rect,
    pub tabs: TopTabs,
    pub command: String,
    pub console_txt: String,
    pub cmd_str_queue: HashMap<String, String>,
    pub console_scroll: u16,
    pub cmd_running: Vec<Command>,
    pub cmd_ended: Vec<Command>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            mode: AppMode::get_mode("normal"),
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec![
                    String::from("Console"),
                    String::from("tab - 2"),
                    String::from("tab - 3"),
                ],
                selection: 0,
            },
            command: String::from(""),
            console_txt: String::from(""),
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
            .field("mode", &self.mode)
            .field("size", &self.size)
            .field("tabs", &self.tabs)
            .finish()
    }
}
