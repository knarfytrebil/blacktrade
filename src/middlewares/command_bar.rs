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
        debug!("3 {:?}", &action);
        match action {
            AppAction::SetMode(ref mode) => {
                let _action = match mode["category"].as_str() {
                    Some("normal") => AppAction::CommandBarSet(String::from("")),
                    Some("command") => AppAction::CommandBarSet(String::from(":")),
                    Some(&_) | None => panic!("Invalid Mode Category"),
                };
                let _ = store.dispatch(_action);
            }
            _ => {}
        }
        next(store, action)
    }
}
