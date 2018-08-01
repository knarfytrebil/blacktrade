use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::action::command::Phase;
use store::app::AppState;

// use std::sync::mpsc;
// use store::events::Event;
// pub struct TxMiddleware {
//     pub tx: mpsc::Sender<Event>,
// }

pub struct CommandMiddleWare { }

impl Middleware<AppState> for CommandMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        match &action {
            &AppAction::Command(Phase::Validate(ref cmd)) => {
                match store.get_state().cmd_reg.contains_key(cmd) {
                    true => {
                        let _ = store.dispatch(AppAction::Command(Phase::Run(cmd.clone())));
                    },
                    false => {
                        let error = format_output!("red", "Error", "Invalid Command");
                        let _ = store.dispatch(AppAction::Error(error));
                    }
                }
            }
            _ => {}
        }
        return next(store, action);
    }
}
