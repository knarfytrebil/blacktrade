use redux::{DispatchFunc, Middleware, Store};
use std::sync::mpsc;
use store::events::Event;
use store::loops::{AppAction, AppState};

pub struct Term {
    pub tx: mpsc::Sender<Event>,
}

impl Middleware<AppState> for Term {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("Called action: {:?}", action);
        let result = next(store, action);
        result
    }
}
