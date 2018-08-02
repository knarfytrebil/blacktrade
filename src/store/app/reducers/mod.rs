mod commands;
mod error;
mod keyboard;
mod mode;

use redux::Reducer;
use store::action::AppAction;
use store::action::command::Phase;
use store::app::AppState;

impl Reducer for AppState {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        match action {
            AppAction::ResizeApp(size) => { self.size = size; }
            AppAction::SetMode(mode) => { 
                let _state = mode::set_mode(self, mode);
            }
            AppAction::ConsoleWrite(line) => {
                self.console_txt.push_str(&line);
            }
            AppAction::Keyboard(key_evt) => {
                Self::key_event_handler(self, key_evt);
            }
            AppAction::Command(Phase::Run(cmd)) => {
                Self::command_handler(self, cmd);
            }
            AppAction::Error(error) => {
                Self::error_handler(self, error);
            }
            _ => { }
        }
        Ok(self.clone())
    }
}

type ReducerFunc = fn(&mut AppState, AppAction) -> Result<&mut AppState, String>;

fn map_reducers (reducers: Vec<ReducerFunc>) -> ReducerFunc {

}
