#![allow(dead_code)]
use termion::{event, color, style};
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
                symbol: String::from("NORM") 
            }, 
            "command" => AppMode { 
                category: ModeCategory::Command,
                symbol: String::from("CTRL") 
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
    pub console_txt: String,
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
                titles: vec![
                    String::from("Console") 
                ],
                selection: 0,
            },
            command: String::from(""),
            console_txt: String::from(""),
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
            AppAction::ConsoleWrite(line) => {
                self.console_txt.push_str(&line);
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
            _ => { error!("Wrong Command Type"); }
        }
    }
    
    fn normal_key_handler(&mut self, evt: event::Key) {
        match evt { 
            event::Key::Char(':') => { self.set_mode("command"); }
            _ => { info!("unimplemented"); }
        }
    }

    fn command_key_handler(&mut self, evt: event::Key) {
        match evt { 
            event::Key::Esc => { 
                self.set_mode("normal");
            },
            event::Key::Backspace => { 
                if self.command == ":" { self.set_mode("normal"); } 
                else { self.command.pop(); }
            },
            // Must be above Char(_char)
            event::Key::Char('\n') => { 
                let cmd = self.command.split_off(1);
                info!("Command Issued: {:?}", cmd);
                let line = String::from("{fg=green [command] }") + &cmd + &String::from("\n");
                self.console_txt.push_str(&line);
            },
            event::Key::Char(_char) => { self.command.push(_char); },
            _ => { }
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
