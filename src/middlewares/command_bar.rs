use actions::AppAction;
use redux::{DispatchFunc, Middleware, Store};
use structs::app::AppState;

pub struct CommandBarMiddleWare {}

impl Middleware<AppState> for CommandBarMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("CommandBar Middleware {:?}", &action);
        if let AppAction::SetMode(ref _mode) = action { }
        next(store, action)
    }
}
