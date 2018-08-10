use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::{AppState, Command};

pub struct ConsoleMiddleWare { }

fn get_index_by_uuid(arr: &Vec<Command>, uuid: &String) -> usize {
    arr.iter().position(|ref r| &r.id == uuid).unwrap()
}


impl Middleware<AppState> for ConsoleMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        match &action {
            // &AppAction::CommandBarEnqueueCmd(ref uuid) => { 
            //     let cmd_str = store.get_state().cmd_str_queue[uuid].clone(); 
            //     let prompt_in = format_output!("green", "In", &cmd_str);
            //     let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            // }
            &AppAction::CommandCreate(ref uuid) => { 
                let cmd_arr = &store.get_state().cmd_running;
                let index = get_index_by_uuid(cmd_arr, uuid);
                let cmd_str = cmd_arr[index].name.clone(); 
                let prompt_in = format_output!("green", "Running", &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            &AppAction::CommandInvalid(ref uuid) => { 
                let cmd_arr = &store.get_state().cmd_running;
                let index = get_index_by_uuid(cmd_arr, uuid);
                let cmd_str = cmd_arr[index].name.clone(); 
                let prompt_in = format_output!("red", "Invalid", &cmd_str);
                let _ = store.dispatch(AppAction::ConsolePush(prompt_in));
            }
            _ => { }
        }
        return next(store, action);
    }

}
