use redux::{DispatchFunc, Middleware, Store};
use actions::AppAction;
use structs::app::{AppState};

pub struct ConsoleMiddleWare { }

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
                let prompt_in = format_output!("green", "In", &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            &AppAction::CommandCreate(ref uuid) => { 
                let cmd_str = store.get_state().cmd_str_queue[uuid].clone();
                let prompt_in = format_output!("green", "Running", &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            &AppAction::CommandInvalid(ref uuid) => { 
                let cmd_str = store.get_state().cmd_str_queue[uuid].clone(); 
                let prompt_in = format_output!("red", "Invalid", &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            _ => { }
        }
        return next(store, action);
    }

}
