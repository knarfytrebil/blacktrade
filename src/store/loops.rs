#![allow(dead_code)]
use termion::event;
use tui::layout::{Rect};

use tui::Terminal;
use tui::backend::Backend;
use tui::backend::MouseBackend;

use store::tab::{TopTabs};
use redux::{Store, Reducer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeCategory { Normal, Command }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppMode {
    pub category: ModeCategory,
    pub symbol: String
}

impl AppMode {
    pub fn get_mode(mode_name: &str) -> AppMode {
        match mode_name {
            "normal" => AppMode { 
                category: ModeCategory::Normal, 
                symbol: String::from("NORMAL") 
            }, 
            "command" => AppMode { 
                category: ModeCategory::Command,
                symbol: String::from("COMMAND") 
            }, 
            &_ => AppMode { 
                category: ModeCategory::Command,
                symbol: String::from("Unknown Mode") 
            }        
        }
    }    
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub mode: AppMode,
    pub size: Rect,
    pub tabs: TopTabs,
    pub command: String,
}

#[derive(Clone)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key), 
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            mode: AppMode::get_mode("normal"),
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec![
                    String::from("Console") 
                ],
                selection: 0,
            },
            command: String::from(""),
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
            },
            AppAction::Keyboard(key_evt) => { 
                Self::key_event_handler(self, key_evt); 
            },
            _ => { }
        }
        Ok(self.clone())
    }
}

// Event Handlers for Key Input
impl AppState {
    fn key_event_handler(&mut self, evt: event::Key) {
        match self.mode.category {
            ModeCategory::Normal => { Self::normal_key_handler(self, evt); },
            ModeCategory::Command => { Self::command_key_handler(self, evt); },
            _ => {}
        }
    }
    
    fn normal_key_handler(&mut self, evt: event::Key) {
        match evt { 
            event::Key::Char(':') => { self.mode = AppMode::get_mode("command"); }
            _ => {}
        }
    }

    fn command_key_handler(&mut self, evt: event::Key) {
        match evt { 
            event::Key::Esc => { self.mode = AppMode::get_mode("normal"); }
            event::Key::Backspace => { self.command.pop(); }
            // Must be above Char(c)
            event::Key::Char('\n') => { self.command.pop(); },
            event::Key::Char(c) => { self.command.push(c); },
            _ => { }
        }
    }
}
