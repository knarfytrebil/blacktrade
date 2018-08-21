use store::app::{AppState};
use reducers::ReducerFn;
use actions::AppAction;

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

pub fn enqueue_cmd() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandBarEnqueueCmd(uuid) => {
                state.cmd_str_queue.insert(uuid.clone(), state.command.split_off(1));
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
