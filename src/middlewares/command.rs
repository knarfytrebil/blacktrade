use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
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
            &AppAction::CommandBarTake => {
                let _ = store.dispatch(AppAction::CommandValidate);
            }
            _ => {}
        }
        return next(store, action);
    }
}

//                 match cmd_reg.contains_key(&cmd) {
//                     true => {
//                         // let _ = store.dispatch();
//                     },
//                     false => {
//                         let error = format_output!("red", "Error", "Invalid Command");
//                         let _ = store.dispatch(AppAction::Error(error));
//                     }
//                 }
