use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::{AppState};

pub struct DebugMiddleWare { }

impl Middleware<AppState> for DebugMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        let state = store.get_state();
        debug!("[ACT]: {:?}", &action);
        debug!("[cmd_str_queue]: {:?}", &state.cmd_str_queue);
        debug!("[cmd_running]: {:?}", &state.cmd_running);
        debug!("[cmd_ended]: {:?}", &state.cmd_ended);
        debug!("[cmd_bar]: {:?}", &state.command);
        return next(store, action);
    }
}
