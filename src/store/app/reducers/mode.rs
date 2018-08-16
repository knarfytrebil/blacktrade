use store::app::{AppState};
use store::action::AppAction;
use store::app::reducers::ReducerFn;

pub fn set() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::SetMode(mode) => {
                state.mode = mode.clone();
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
