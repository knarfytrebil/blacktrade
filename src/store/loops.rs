#![allow(dead_code)]
use termion::event;
use tui::layout::{Rect};

use tui::Terminal;
use tui::backend::Backend;
use tui::backend::MouseBackend;

use store::tab::{TopTabs};
use redux::{Store, Reducer};


#[derive(Clone, Debug)]
pub struct AppState<'a> {
    pub mode: i8,
    pub size: Rect,
    pub tabs: TopTabs<'a>,
}

impl<'a> AppState<'a> {
    pub fn new() -> AppState<'a> {
        AppState {
            // mode: 0=NORMAL 1=COMMAND
            mode: 0,
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec!["CMD", "Poloniex"],
                selection: 0,
            },
        }
    }
}

#[derive(Clone)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key), 
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
            _ => {

            }
        }
        Ok(self.clone())
    }
}

// Event Handlers for Key Input
impl<'a> AppState<'a> {
    fn key_event_handler(&mut self, evt: event::Key) {
        match self.mode {
            0 => { Self::normal_key_handler(self, evt); },
            1 => { Self::command_key_handler(self, evt); },
            _ => {}
        }
    }
    
    fn normal_key_handler(&mut self, evt: event::Key) {

    }

    fn command_key_handler(&mut self, evt: event::Key) {

    }
}
