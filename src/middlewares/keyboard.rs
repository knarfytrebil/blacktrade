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
        match &action {
            &AppAction::Keyboard(key) => {
                debug!("Called Keyboard Action: {:?}", action);
                let _state = store.get_state();
                let _action = get_key_action(action, _state);
            }
            _ => {}
        }
        return next(store, action);
    }
}

fn get_key_action(_action: AppAction, _state: AppState) -> AppAction {
    match _state.mode.category {
        ModeCategory::Normal => get_normal_key_action(_action, _state),
        ModeCategory::Command => get_command_key_action(_action, _state)
    }
}

fn get_normal_key_action (action: AppAction, state: AppState) -> AppAction {
    match action {
        AppAction::Keyboard(Key::Char(':')) => AppAction::SetMode(ModeCategory::Command),
        _ => action
    }
}   


fn get_command_key_action (action: AppAction, mut state: AppState) -> AppAction {
    match action {
        AppAction::Keyboard(Key::Char('\n')) => {
            let cmd = state.command.split_off(1);
            // let prompt_in = format_output!("green", "In", &cmd);
            // let _ = store.dispatch(AppAction::ConsoleWrite(prompt_in));
            AppAction::Command(Phase::Validate(cmd))
        }
        AppAction::Keyboard(Key::Char(_char)) => AppAction::CommandBarAppend(_char.to_string()),
        AppAction::Keyboard(Key::Esc) => AppAction::SetMode(ModeCategory::Normal),
        _  => action 
    }
}
