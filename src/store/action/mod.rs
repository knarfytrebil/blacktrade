pub mod command;
use termion::event;
use tui::layout::Rect;
use store::app::{AppMode};

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    ConsoleWrite(String),
    Command(command::Phase),
    CommandBarAppend(String),
    Error(String),
    SetMode(AppMode),
}
