use actions::AppAction;
use reducers::ReducerFn;
use serde_json::Value;
use structs::app::AppState;

pub fn set() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            if let AppAction::SetMode(ref mode) = action {
                let _action = match mode["category"].as_str() {
                    Some("normal") => state.json_store["command"] = Value::from(""),
                    Some("command") => state.json_store["command"] = Value::from(":"),
                    Some(&_) | None => panic!("Invalid Mode Category"),
                };
            };
            Ok(state)
        },
    )
}

pub fn push() -> Box<ReducerFn> {
    Box::new(
        |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandBarPush(_char) => {
                    let value = state.json_store["command"]
                        .as_str()
                        .expect("command is not str");
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
                    let value = state.json_store["command"]
                        .as_str()
                        .expect("command is not str");
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
                    let value = state.json_store["command"]
                        .as_str()
                        .expect("command is not str");
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
