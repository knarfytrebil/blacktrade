use structs::app::{AppState};
use reducers::ReducerFn;
use actions::AppAction;

pub fn set() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::SetMode(mode) => {
                state.json_store["mode"] = mode.clone();
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
