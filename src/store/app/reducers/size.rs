use store::app::{AppState};
use store::action::AppAction;
use store::app::reducers::ReducerFn;

pub fn set() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::ResizeApp(size) => {
                state.size = size.clone(); 
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
