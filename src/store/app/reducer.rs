use redux::Reducer;
use store::action::AppAction;
use store::app::AppState;

impl Reducer for AppState {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        match action {
            AppAction::ResizeApp(size) => {
                self.size = size;
            }
            AppAction::Keyboard(key_evt) => {
                Self::key_event_handler(self, key_evt);
            }
            AppAction::ConsoleWrite(line) => {
                self.console_txt.push_str(&line);
            }
        }
        Ok(self.clone())
    }
}
