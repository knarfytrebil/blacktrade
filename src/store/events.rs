use store::app::AppState;
use termion::event;

pub enum Event {
    Input(event::Key),
    Render(AppState),
}
