pub mod command;
// use reducers::CommandGen;
use structs::app::events;
use structs::app::AppMode;
use termion::event;
use tui::layout::Rect;

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    CommandInvalid(String),
    CommandCreate(String),
//    CommandRun {
//        func: CommandGen,
//        uuid: String,
//    },
    CommandEnd {
        uuid: String,
        success: bool,
        reason: String,
    },
    CommandConsume(String),
    CommandBarPush(char),
    CommandBarSet(String),
    CommandBarEnqueueCmd(String),
    ConsolePush(String),
    SetMode(AppMode),
}

impl AppAction {
    pub fn into_event(self) -> events::Event {
        events::Event::Dispatch(self)
    }
}
