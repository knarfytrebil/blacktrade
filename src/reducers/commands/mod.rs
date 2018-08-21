use structs::app::{AppState, Command};
use reducers::ReducerFn;
use actions::AppAction;

fn get_index_by_uuid(arr: &Vec<Command>, uuid: &String) -> usize {
    arr.iter().position(|ref r| &r.id == uuid).unwrap()
}

// Reducer Functions
pub fn run_command() -> Box<ReducerFn> {
    Box::new(move |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandCreate(uuid) => {
                let cmd_str_index = get_index_by_uuid(&state.cmd_running, uuid);
                let cmd_str = &state.cmd_running[cmd_str_index].name.clone();
                &state.cmd_reg[cmd_str](&mut state, "test".to_string());
            }
            _ => {}
        }
        Ok(state)
    })
}

pub fn create(failed: bool) -> Box<ReducerFn> {
    Box::new(move |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandCreate(uuid) | AppAction::CommandInvalid(uuid) => {
                match state.cmd_str_queue.remove(uuid) {
                    Some(cmd_str) => {
                        let cmd_obj = Command::new(cmd_str.clone(), uuid.clone(), failed);
                        match &failed {
                            true => { state.cmd_ended.push(cmd_obj) }
                            false => { state.cmd_running.push(cmd_obj) }
                        }
                    }
                    None => { debug!("No Such Entry!!!!!!"); }
                }
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}

pub fn end(uuid: String) -> Box<ReducerFn> {
    Box::new(move |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandRun{ func, uuid } => {
                let cmd_str_index = get_index_by_uuid(&state.cmd_running, uuid);
                let cmd = state.cmd_running.remove(cmd_str_index);
                let prompt_in = format_output!("green", "Ended", &cmd.name.clone());
                state.cmd_ended.push(cmd);
                state.console_txt.push_str(&prompt_in);
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}

pub fn helloworld() -> Box<ReducerFn> {
    Box::new(move |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        state.console_txt.push_str(&"hello world\n".to_string());
        Ok(state)
    })
}
