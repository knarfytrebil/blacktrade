use termion::event;
use store::loops::{AppState};

pub enum Event {
    Input(event::Key),
    Render(AppState),
}
