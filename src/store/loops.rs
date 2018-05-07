#![allow(dead_code)]
use tui::layout::{Rect};

use tui::Terminal;
use tui::backend::Backend;
use tui::backend::MouseBackend;

use store::tab::{TopTabs};
use redux::{Store, Reducer};


#[derive(Clone, Debug)]
pub struct App<'a> {
    pub size: Rect,
    pub tabs: TopTabs<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
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
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App::new()
    }
}

impl<'a> Reducer for App<'a> {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        match action {
            AppAction::ResizeApp(size) => {
                self.size = size;
            },
        }
        Ok(self.clone())
    }
}
