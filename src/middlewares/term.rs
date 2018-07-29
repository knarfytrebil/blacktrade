use std::sync::mpsc;
use termion::event::Key;
use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::AppState;
use store::events::Event;

pub struct Term {
    pub tx: mpsc::Sender<Event>,
}

impl Middleware<AppState> for Term {
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
                let line = format_output!("green", "In", &cmd);
                debug!("Command Issued {:?}", cmd);
                let _ = store.dispatch(AppAction::ConsoleWrite(line));
                let _ = store.dispatch(AppAction::Command(cmd));
            }
            _ => { }
        }
        next(store, action)
    }
}
