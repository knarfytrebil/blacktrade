use std::sync::mpsc;
use redux::{Store, Reducer, Middleware, DispatchFunc};
use store::loops::{AppState, AppAction};
use store::events::Event;

pub struct Term {
    pub tx: mpsc::Sender<Event>
}

impl Middleware<AppState> for Term {
    fn dispatch(
        &self, store: &Store<AppState>, 
        action: AppAction, 
        next: &DispatchFunc<AppState>
    ) -> Result<AppState, String> {
        // debug!("Called action: {:?}", action);
        let result = next(store, action);
        result
    }
}
