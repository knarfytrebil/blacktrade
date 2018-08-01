use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::action::command::Phase;
use store::app::{AppState, ModeCategory};
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
        let mut _state = store.get_state();
        match &action {
            &AppAction::Keyboard(Key::Char('\n')) => {
                let cmd = _state.command.split_off(1);
                let prompt_in = format_output!("green", "In", &cmd);
                let _ = store.dispatch(AppAction::ConsoleWrite(prompt_in));
                let _ = store.dispatch(AppAction::Command(Phase::Validate(cmd)));
            }
            &AppAction::Keyboard(Key::Char(':')) => {

            }
            &AppAction::Keyboard(Key::Esc) => {
            }
            _ => {}
        }
        return next(store, action);
    }
}

fn get_mode_change_action (state: AppState, key: Key) -> AppAction {
    match state.mode.category {
        ModeCategory::Normal => {
            match key {
                Key::Char(':') => AppAction::SetMode(ModeCategory::Command),
                _ => {},
            }
        }
        ModeCategory::Command => {
            match key {
                Key::Char(_char) => AppAction::CommandBarAppend(_char),
                Key::Esc => AppAction::SetMode(ModeCategory::Normal),
                _  => {}
            }

        }
    }   
}
