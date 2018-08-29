use actions::AppAction;
use reducers::ReducerFn;
use structs::app::{AppState, Command};

fn get_index_by_uuid(arr: &[Command], uuid: &str) -> usize {
    arr.iter().position(|ref r| r.id == uuid).unwrap()
}

// Reducer Functions
pub fn run_command() -> Box<ReducerFn> {
    Box::new(
        move |state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandCreate(uuid) => {
                    let cmd_str_index = get_index_by_uuid(&state.cmd_running, uuid);
                    let cmd_str = &state.cmd_running[cmd_str_index].name.clone();
                    //                &state.cmd_reg[cmd_str](&mut state, "test".to_string());
                }
                _ => {}
            }
            Ok(state)
        },
    )
}

pub fn do_nothing() -> Box<ReducerFn> {
    Box::new(move |state: AppState, _action: &AppAction| -> Result<AppState, String> { Ok(state) })
}

pub fn create(failed: bool) -> Box<ReducerFn> {
    Box::new(
        move |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandCreate(uuid) | AppAction::CommandInvalid(uuid) => {
                    match state.cmd_str_queue.remove(uuid) {
                        Some(cmd_str) => {
                            let cmd_obj = Command::new(cmd_str.clone(), uuid.clone(), failed);
                            match &failed {
                                true => state.cmd_ended.push(cmd_obj),
                                false => state.cmd_running.push(cmd_obj),
                            }
                        }
                        None => {
                            debug!("No Such Entry!!!!!!");
                        }
                    }
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}

pub fn end(uuid: String, success: bool) -> Box<ReducerFn> {
    Box::new(
        move |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
            match action {
                AppAction::CommandEnd {
                    uuid,
                    success,
                    reason,
                } => {
                    let cmd_str_index = get_index_by_uuid(&state.cmd_running, uuid);
                    let mut cmd = state.cmd_running.remove(cmd_str_index);
                    cmd.failed = !success;
                    state.cmd_ended.push(cmd);
                    Ok(state)
                }
                _ => Ok(state),
            }
        },
    )
}
