pub mod command;
use termion::event;
use tui::layout::Rect;

#[derive(Clone, Debug)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    ConsoleWrite(String),
    Command(command::Phase),
    Error(String),
}
