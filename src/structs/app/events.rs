use actions::AppAction;
use structs::app::AppState;

pub enum Event {
    Render(AppState),
    Dispatch(AppAction),
    Exit,
}
