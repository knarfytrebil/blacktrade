#![allow(dead_code)]
use tui::layout::{Rect};
use store::tab::{TopTabs};

pub struct App<'a> {
    pub size: Rect,
    pub tabs: TopTabs<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec!["Poloniex", "Logs"],
                selection: 0,
            }
        }
    }
}
