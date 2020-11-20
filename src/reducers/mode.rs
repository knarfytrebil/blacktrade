use actions::AppAction;
use reducers::ReducerFn;
use structs::app::AppState;

pub fn set() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::SetMode(mode) => {
                    state.json_store["mode"] = mode.clone();
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}
