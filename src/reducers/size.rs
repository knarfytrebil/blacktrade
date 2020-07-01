use actions::AppAction;
use reducers::ReducerFn;
use structs::app::AppState;

pub fn set() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::ResizeApp(size) => {
                    state.size = *size;
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}
