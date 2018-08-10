use uuid::Uuid;
use termion::event::Key;
use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::{AppState, AppMode, ModeCategory};

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
        debug!("[ACT-KBD]: {:?}", &action);
        match &action {
            &AppAction::Keyboard(key) => {
                let _state = store.get_state();
                match get_key_action(key, _state) {
                    Ok(_action) => { 
                        debug!("[PreDispatch]: {:?}", &_action);
                        let result = store.dispatch(_action); 
                        debug!("[Dispatch Result]: {:?}", result);
                    }
                    Err(err) => { debug!("[ERR] {:?}", err) }
                }
            }
            _ => {}
        }
        return next(store, action);
    }
}

fn get_key_action(_key: Key, _state: AppState) -> Result<AppAction, String> {
    match _state.mode.category {
        ModeCategory::Normal => normal_key(_key, _state),
        ModeCategory::Command => command_key(_key, _state)
    }
}

fn normal_key (_key: Key, _state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Char(':') => Ok(AppAction::SetMode(AppMode::get_mode("command"))),
        _ => Err(String::from("Key not Implemented"))
    }
}   

// let prompt_in = format_output!("green", "In", &cmd);
// let _ = store.dispatch(AppAction::ConsoleWrite(prompt_in));

fn command_key (_key: Key, mut _state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Esc => Ok(AppAction::SetMode(AppMode::get_mode("normal"))),
        Key::Char('\n') => { Ok(AppAction::CommandBarEnqueueCmd(Uuid::new_v4().simple().to_string())) }
        Key::Char(_char) => { Ok(AppAction::CommandBarPush(_char)) }
        _  => { Err(String::from("Key not Implemented")) }
    }
}
