use actions::AppAction;
use reducers::CommandGen;
use structs::app::AppState;

pub enum Event {
    Render(AppState),
    Dispatch(AppAction),
    Exit,
}
