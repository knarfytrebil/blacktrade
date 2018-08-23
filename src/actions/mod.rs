pub mod command;
use termion::event;
use tui::layout::Rect;
use structs::app::{AppMode};
use reducers::{CommandGen};

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    CommandInvalid(String),
    CommandCreate(String),
    CommandRun {
        func: CommandGen,
        uuid: String
    },
    CommandFail(String),
    CommandConsume(String),
    CommandBarPush(char),
    CommandBarSet(String),
    CommandBarEnqueueCmd(String),
    ConsolePush(String),
    SetMode(AppMode),
}
