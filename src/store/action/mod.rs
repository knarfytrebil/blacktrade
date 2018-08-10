pub mod command;
use termion::event;
use tui::layout::Rect;
use store::app::{AppMode};

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    CommandInvalid(String),
    CommandCreate(String),
    CommandFail(String),
    CommandBarPush(char),
    CommandBarSet(String),
    CommandBarEnqueueCmd(String),
    TestB(String),
    ConsolePush(String),
    Error(String),
    SetMode(AppMode),
}
