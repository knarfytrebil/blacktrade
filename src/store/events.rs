use store::loops::AppState;
use termion::event;

pub enum Event {
    Input(event::Key),
    Render(AppState),
}
