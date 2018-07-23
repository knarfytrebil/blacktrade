#[derive(Clone, Debug)]
pub enum AppAction {
    ResizeApp(Rect),
    Keyboard(event::Key),
    ConsoleWrite(String),
}
