use store::app::{AppState};
use reducers::ReducerFn;
use actions::AppAction;

pub fn push() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::ConsolePush(line) => {
                state.console_txt.push_str(line);
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
