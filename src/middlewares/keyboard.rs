use actions::AppAction;
use redux::{DispatchFunc, Middleware, Store};
use structs::app::events::Key as SerializableKey;
use structs::app::AppState;
use termion::event::Key;
use utils::app::to_unserializable;
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
        if let AppAction::Keyboard(key) = action {
            let _state = store.get_state();
            match get_key_action(key, _state) {
                Ok(_action) => {
                    let _ = store.dispatch(_action);
                }
                Err(err) => debug!("[ERR] {:?}", err),
            }
        };
        next(store, action)
    }
}

fn get_key_action(_key: SerializableKey, _state: AppState) -> Result<AppAction, String> {
    let key_event = to_unserializable(_key);
    match _state.json_store["mode"].as_str() {
        Some("normal") => normal_key(key_event, _state),
        Some("command") => command_key(key_event, _state),
        Some(&_) | None => panic!("Unknown Category !"),
    }
}

fn normal_key(_key: Key, _state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Char(':') => {
            let action = AppAction::SetMode("command".to_string());
            Ok(action)
        }
        _ => Err(String::from("There is no settings for this key yet")),
    }
}

fn command_key(_key: Key, mut _state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Esc => {
            let action = AppAction::SetMode("normal".to_string());
            Ok(action)
        }
        Key::Backspace => Ok(AppAction::CommandBarPop(1)),
        Key::Char('\n') => Ok(AppAction::CommandBarEnqueueCmd(Uuid::new_v4().to_string())),
        Key::Char(_char) => Ok(AppAction::CommandBarPush(_char)),
        _ => Err(String::from("Key not Implemented")),
    }
}
