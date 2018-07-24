use redux::{DispatchFunc, Middleware, Store};
use std::sync::mpsc;
use store::action::AppAction;
use store::app::AppState;
use store::events::Event;

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
