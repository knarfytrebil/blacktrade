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
    pub size: Rect,
    pub tabs: TopTabs<'a>,
}

impl<'a> AppState<'a> {
    pub fn new() -> AppState<'a> {
        AppState {
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
            _ => {

            }
        }
        Ok(self.clone())
    }
}
