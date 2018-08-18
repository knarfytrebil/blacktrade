use store::app::AppState;
use termion::event;
use store::app::reducers::{CommandGen};

pub enum Event {
    Input(event::Key),
    Render(AppState),
    CommandQueued(String),
    CommandRun {
        func: CommandGen,
        uuid: String
    },
    Exit, 
}

