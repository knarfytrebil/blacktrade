use actions::AppAction;
use reducers::ReducerFn;
use structs::app::AppState;
use serde_json::{Value};

pub fn set() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandBarSet(str_ref) => {
                    state.json_store["command"] = Value::String(str_ref.to_string());
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}

pub fn push() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandBarPush(_char) => {

                    let value = state.json_store["command"].as_str().expect("command is not str");
                    let mut process_value = value.to_string();

                    process_value.push(*_char);

                    state.json_store["command"] = Value::String(process_value);
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}

pub fn pop() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandBarPop(_pop_index) => {
                    let value = state.json_store["command"].as_str().expect("command is not str");
                    let mut process_value = value.to_string();

                    if process_value.len() > 1 {
                        process_value.pop();
                        state.json_store["command"] = Value::String(process_value);
                    }
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}

pub fn enqueue_cmd() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandBarEnqueueCmd(uuid) => {
                    let value = state.json_store["command"].as_str().expect("command is not str");
                    let mut process_value = value.to_string();

                    state
                        .cmd_str_queue
                        .insert(uuid.clone(), process_value.split_off(1));

                    state.json_store["command"] = Value::String(process_value);
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}
