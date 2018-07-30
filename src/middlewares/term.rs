use redux::{DispatchFunc, Middleware, Store};
use std::sync::mpsc;
use store::action::AppAction;
use store::app::AppState;
use store::events::Event;
use termion::event::Key;

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
        handle_action(store, &action);
        return next(store, action);
    }
}

fn validate_action(state: &AppState, action: &AppAction) -> AppAction {
    match *action {
        AppAction::Command(ref cmd) => match state.cmd_reg.contains_key(cmd) {
            true => return action.clone(),
            false => {
                let error = format_output!("red", "Error", "Unregistered command");
                return AppAction::Error(error);
            }
        },

        _ => {
            return action.clone();
        }
    }
}

#[allow(unused_must_use)]
fn handle_action(store: &Store<AppState>, action: &AppAction) {
    match &action {
        &AppAction::Keyboard(Key::Char('\n')) => {
            let cmd = store.get_state().command.split_off(1);
            let prompt_in = format_output!("green", "In", &cmd);
            store.dispatch(validate_action(
                &store.get_state(),
                &AppAction::ConsoleWrite(prompt_in),
            ));
            store.dispatch(validate_action(
                &store.get_state(),
                &AppAction::Command(cmd),
            ));
        }
        _ => {}
    }
}
