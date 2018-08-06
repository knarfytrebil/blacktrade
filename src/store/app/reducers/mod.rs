mod commands;
mod error;
mod keyboard;
// mod mode;

use redux::Reducer;
use store::action::AppAction;
use store::action::command::Phase;
use store::app::AppState;

type ReducerFn = Fn(AppState, AppAction) -> Result<AppState, String>;
type ReducerArray = Vec<Box<ReducerFn>>;

impl Reducer for AppState {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        let set_mode = Box::new(|mut state: AppState, action: AppAction| -> Result<AppState, String> {
            match action {
                AppAction::SetMode(mode) => {
                    state.mode = mode;
                    Ok(state)
                }
                _ => { Ok(state) }
            }

        });
        match action {
            AppAction::ResizeApp(size) => { self.size = size; }
            AppAction::SetMode(mode) => { 
                let reducers: ReducerArray = vec![set_mode];
                let _state = combined_reducer(reducers)(self.clone(), action.clone()).unwrap();
                // let _state = mode::set_mode(self, mode);
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

fn combined_reducer(reducers: Vec<Box<ReducerFn>>) -> Box<ReducerFn> {
    Box::new(move |mut state, action| {
        for reducer in &reducers {
            state = reducer(state, action).unwrap() 
        }
        Ok(state)
    })
}
