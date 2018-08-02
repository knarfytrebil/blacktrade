use store::app::{AppState, AppMode};

pub fn set_mode(state: &mut AppState, mode: AppMode) -> &mut AppState {
    state.mode = mode; 
    state
}
