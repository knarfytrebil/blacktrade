use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::action::command::Phase;
use store::app::AppState;
use termion::event::Key;

// use std::sync::mpsc;
// use store::events::Event;
// pub struct TxMiddleware {
//     pub tx: mpsc::Sender<Event>,
// }

pub struct KeyboardMiddleWare { }

impl Middleware<AppState> for KeyboardMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("Called action: {:?}", action);
        match &action {
            &AppAction::Keyboard(Key::Char('\n')) => {
                let cmd = store.get_state().command.split_off(1);
                let prompt_in = format_output!("green", "In", &cmd);
                let _ = store.dispatch(AppAction::ConsoleWrite(prompt_in));
                let _ = store.dispatch(AppAction::Command(Phase::Validate(cmd)));
            }
            _ => {}
        }
        return next(store, action);
    }
}
