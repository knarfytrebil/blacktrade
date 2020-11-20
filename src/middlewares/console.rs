use actions::AppAction;
use redux::{DispatchFunc, Middleware, Store};
use structs::app::AppState;

pub struct ConsoleMiddleWare {}

impl Middleware<AppState> for ConsoleMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("4 {:?}", &action);
        match &action {
            &AppAction::CommandConsume(ref uuid) => {
                match get_action_in_queue(uuid.to_string(), store) {
                    Ok(action) => {
                        let _ = store.dispatch(action);
                    }
                    Err(err_msg) => {
                        debug!("ERROR {:?}", &err_msg);
                    }
                }
            }
            &AppAction::CommandCreate(ref uuid) | &AppAction::CommandInvalid(ref uuid) => {
                let cmd_str = store.get_state().cmd_str_queue[uuid].clone();
                let prompt_in = format_output!("white", uuid, &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            &AppAction::CommandEnd {
                ref uuid,
                ref success,
                ref reason,
            } => {
                let prompt_in = match success {
                    true => format_output!("green", uuid, "Process Terminated"),
                    false => format_output!("red", uuid, reason),
                };
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            _ => {}
        }
        next(store, action)
    }
}

fn get_action_in_queue(uuid: String, _store: &Store<AppState>) -> Result<AppAction, String> {
    match _store.get_state().cmd_str_queue.contains_key(&uuid) {
        true => {
            let cmd_str = _store.get_state().cmd_str_queue[&uuid].clone();
            let prompt_in = format_output!("green", ">>>", &cmd_str);
            Ok(AppAction::ConsolePush(prompt_in))
        }
        false => Err(String::from("Command Not Found")),
    }
}
