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
pub struct AppMode<'a> {
    pub category: ModeCategory,
    pub symbol: &'a str,
}

#[derive(Clone, Debug)]
pub struct AppState<'a> {
    pub mode: AppMode<'a>,
    pub size: Rect,
    pub tabs: TopTabs<'a>,
    pub command: String,
}

#[derive(Clone)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key), 
}

const NORMALMODE: AppMode = AppMode { 
    category: ModeCategory::Normal, 
    symbol: "NORMAL"
};

const COMMANDMODE: AppMode = AppMode { 
    category: ModeCategory::Command, 
    symbol: "COMMAND"
};

impl<'a> AppState<'a> {
    pub fn new() -> AppState<'a> {
        AppState {
            mode: NORMALMODE,
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec!["Console", "Poloniex"],
                selection: 0,
            },
            command: String::from(""),
        }
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        AppState::new()
    }
}

impl<'a> Reducer for AppState<'a> {
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
impl<'a> AppState<'a> {
    fn key_event_handler(&mut self, evt: event::Key) {
        match self.mode {
            NORMALMODE => { Self::normal_key_handler(self, evt); },
            COMMANDMODE => { Self::command_key_handler(self, evt); },
            _ => {}
        }
    }
    
    fn normal_key_handler(&mut self, evt: event::Key) {
        match evt { 
            event::Key::Char(':') => { self.mode = COMMANDMODE; }
            _ => {}
        }
    }

    fn command_key_handler(&mut self, evt: event::Key) {
        match evt { 
            event::Key::Esc => { self.mode = NORMALMODE; }
            event::Key::Backspace => { self.command.pop(); }
            // Must be above Char(c)
            event::Key::Char('\n') => { self.command.pop(); },
            event::Key::Char(c) => { self.command.push(c); },
            _ => { }
        }
    }
}
