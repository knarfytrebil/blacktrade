use actions::AppAction;
use reducers::ReducerFn;
use serde_json::Value;
use structs::app::AppState;

pub fn push() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::ConsolePush(line) => {
                    let value = state.json_store["console_output_lines"]
                        .as_array()
                        .expect("command is not array");
                    let mut process_value = value.to_vec();
                    process_value.push(Value::String(line.to_string()));
                    state.json_store["console_output_lines"] = Value::Array(process_value);
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}
