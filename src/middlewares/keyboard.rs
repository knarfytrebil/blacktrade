use actions::AppAction;
use redux::{DispatchFunc, Middleware, Store};
use structs::app::{AppMode, AppState, ModeCategory};
use termion::event::Key;
use uuid::Uuid;

pub struct KeyboardMiddleWare {}

impl Middleware<AppState> for KeyboardMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("2 {:?}", &action);
        match action {
            AppAction::Keyboard(key) => {
                let _state = store.get_state();
                match get_key_action(key, _state) {
                    Ok(_action) => {
                        let _ = store.dispatch(_action);
                    }
                    Err(err) => debug!("[ERR] {:?}", err),
                }
            }
            _ => {}
        }
        next(store, action)
    }
}

fn get_key_action(_key: Key, _state: AppState) -> Result<AppAction, String> {
    match _state.mode.category {
        ModeCategory::Normal => normal_key(_key, _state),
        ModeCategory::Command => command_key(_key, _state),
    }
}

fn normal_key(_key: Key, _state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Char(':') => Ok(AppAction::SetMode(AppMode::get_mode("command"))),
        _ => Err(String::from("There is no settings for this key yet")),
    }
}

fn command_key(_key: Key, mut _state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Esc => Ok(AppAction::SetMode(AppMode::get_mode("normal"))),
        Key::Backspace => Ok(AppAction::CommandBarPop(1)),
        Key::Char('\n') => Ok(AppAction::CommandBarEnqueueCmd(Uuid::new_v4().to_string())),
        Key::Char(_char) => Ok(AppAction::CommandBarPush(_char)),
        _ => Err(String::from("Key not Implemented")),
    }
}
