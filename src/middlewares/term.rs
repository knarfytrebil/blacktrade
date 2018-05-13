use redux::{Store, Reducer, Middleware, DispatchFunc};
use store::loops::{AppState, AppAction};

pub struct Term;

impl Middleware<AppState> for Term {
    fn dispatch(
        &self, store: &Store<AppState>, 
        action: AppAction, 
        next: &DispatchFunc<AppState>) -> Result<AppState, String> {

        let result = next(store, action);
        result
    }
}
