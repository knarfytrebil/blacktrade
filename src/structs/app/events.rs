use structs::app::{AppState};
use actions::AppAction;
use reducers::{CommandGen};

pub enum Event {
    Render(AppState),
    Dispatch(AppAction),
    Exit, 
}
