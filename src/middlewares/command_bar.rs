use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::{AppState, ModeCategory};

pub struct CommandBarMiddleWare { }

impl Middleware<AppState> for CommandBarMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("[ACT]: {:?}", &action);
        match &action {
            &AppAction::SetMode(ref mode) => {
                let _action = match mode.category {
                    ModeCategory::Normal => AppAction::CommandBarSet(String::from("")),
                    ModeCategory::Command => AppAction::CommandBarSet(String::from(":"))
                };
                let _ = store.dispatch(_action);
            }
            _ => {}
        }
        return next(store, action);
    }
}
