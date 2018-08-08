pub mod command;
use termion::event;
use tui::layout::Rect;
use store::app::{AppMode};

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    CommandValidate,
    CommandRun(String),
    CommandBarPush(char),
    CommandBarSet(String),
    CommandBarTake,
    ConsolePush(String),
    Error(String),
    SetMode(AppMode),
}
