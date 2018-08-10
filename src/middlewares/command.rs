use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::AppState;

pub struct CommandMiddleWare { }

impl Middleware<AppState> for CommandMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("[ACT]: {:?}", &action);
        match &action {
            // &AppAction::CommandBarEnqueueCmd(ref uuid) => {
            //     let state = store.get_state();
            //     let command = &state.cmd_str_queue[uuid].clone();
            //     let _action = match state.cmd_reg.contains_key(command) {
            //         true => { AppAction::CommandCreate(uuid.to_string()) }
            //         false => { AppAction::CommandInvalid(uuid.to_string()) }
            //     };
            //     let _ = store.dispatch(_action);
            // }
            _ => {}
        }
        next(store, action)
    }
}
