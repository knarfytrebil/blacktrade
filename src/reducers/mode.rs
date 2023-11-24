use actions::AppAction;
use reducers::ReducerFn;
use structs::app::AppState;
use serde_json::Value;

pub fn set() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::SetMode(mode) => {
                    state.json_store["mode"] = Value::from(mode.as_str());
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}
