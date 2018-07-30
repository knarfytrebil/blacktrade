use std::fmt;
use std::collections::HashMap;
use tui::layout::Rect;
use store::ui::TopTabs;
use store::app::AppMode;
use store::app::structs::CmdCallback;

#[derive(Clone)]
pub struct AppState {
    pub mode: AppMode,
    pub size: Rect,
    pub tabs: TopTabs,
    pub command: String,
    pub console_txt: String,
    pub exiting: bool,
    pub cmd_reg: HashMap<String, CmdCallback>,
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
            cmd_reg: HashMap::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let mut state = AppState::new();
        state.cmd_reg.insert("test_cb".to_string(), Self::test_cb);
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
