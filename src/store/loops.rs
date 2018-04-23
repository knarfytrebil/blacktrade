#![allow(dead_code)]
use tui::layout::{Rect};

struct App<'a> {
    size: Rect,
    tabs: TopTabs<'a>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec!["Poloniex", "Logs"],
                selection: 0,
            }
        }
    }
}
