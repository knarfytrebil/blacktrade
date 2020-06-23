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
                let cmd_str = store.get_state().cmd_str_queue[uuid].clone();
                let prompt_in = format_output!("green", ">>>", &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            &AppAction::CommandCreate(ref uuid) => {
                let cmd_str = store.get_state().cmd_str_queue[uuid].clone();
                let prompt_in = format_output!("green", uuid, &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            &AppAction::CommandInvalid(ref uuid) => {
                let cmd_str = store.get_state().cmd_str_queue[uuid].clone();
                let prompt_in = format_output!("red", uuid, &cmd_str);
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
