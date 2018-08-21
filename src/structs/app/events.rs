use store::app::AppState;
use termion::event;
use reducers::{CommandGen};

pub enum Event {
    Input(event::Key),
    Render(AppState),
    CommandQueued(String),
    CommandRun {
        func: CommandGen,
        uuid: String
    },
    ConsolePush(String),
    Exit, 
}

