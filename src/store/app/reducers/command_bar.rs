use store::app::{AppState};
use store::action::AppAction;
use store::app::reducers::ReducerFn;

pub fn set() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandBarSet(str_ref) => {
                state.command = str_ref.to_string();
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}

pub fn push() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandBarPush(_char) => {
                state.command.push(*_char);
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}

pub fn take() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandBarTake => {
                state.cmd_issued.push(state.command.split_off(1));
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
