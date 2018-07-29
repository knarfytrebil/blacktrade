mod commands;
mod keyboard;
use redux::Reducer;
use store::action::AppAction;
use store::app::AppState;

impl Reducer for AppState {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        match action {
            AppAction::ResizeApp(size) => { self.size = size; }
            AppAction::ConsoleWrite(line) => { self.console_txt.push_str(&line); }
            AppAction::Keyboard(key_evt) => { Self::key_event_handler(self, key_evt); }
            AppAction::Command(command) => { Self::command_handler(self, command); }
        }
        Ok(self.clone())
    }
}
