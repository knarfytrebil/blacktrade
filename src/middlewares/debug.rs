use actions::AppAction;
use redux::{DispatchFunc, Middleware, Store};
use structs::app::AppState;

pub struct DebugMiddleWare {}

impl Middleware<AppState> for DebugMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("1 {:?}", &action);
        let state = store.get_state();
        debug!("[ACT]: {:?}", &action);
        debug!("[cmd_str_queue]: {:?} Items", &state.cmd_str_queue.len());
        debug!("[cmd_running]: {:?} Items", &state.cmd_running.len());
        debug!("[json_store]: {:?}", &state.json_store);
        next(store, action)
    }
}
