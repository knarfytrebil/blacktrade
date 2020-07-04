mod command_bar;
pub mod commands;
mod console;
mod error;
mod keyboard;
mod mode;
mod size;

use actions::AppAction;
use redux::Reducer;
use structs::app::AppState;

pub type ReducerFn = dyn Fn(AppState, &AppAction) -> Result<AppState, String>;
pub type CommandGen = fn() -> Box<ReducerFn>;
type ReducerArray = Vec<Box<ReducerFn>>;

impl Reducer for AppState {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        debug!("REDUCED {:?}", &action);
        let reducers: ReducerArray = match action {
            AppAction::ResizeApp(_) => vec![size::set()],
            AppAction::SetMode(_) => vec![mode::set()],
            AppAction::ConsolePush(_) => vec![console::push()],
            AppAction::CommandBarPush(_) => vec![command_bar::push()],
            AppAction::CommandBarSet(_) => vec![command_bar::set()],
            AppAction::CommandBarEnqueueCmd(_) => vec![command_bar::enqueue_cmd()],
            AppAction::CommandCreate(_) => vec![commands::create(false)],
            AppAction::CommandInvalid(_) => vec![commands::create(true)],
            AppAction::CommandEnd {
                ref uuid,
                success,
                reason: _,
            } => vec![commands::end(uuid.to_string(), success)],

            // AppAction::Keyboard(key_evt) => {
            //     Self::key_event_handler(self, key_evt);
            // }
            // AppAction::Error(error) => {
            //     Self::error_handler(self, error);
            //     commands::end(uuid.to_string())
            // }
            _ => vec![],
        };
        let _state = combined_reducer(reducers)(self.clone(), &action).unwrap();
        Ok(_state)
    }
}

fn combined_reducer(reducers: Vec<Box<ReducerFn>>) -> Box<ReducerFn> {
    Box::new(move |mut state, action| {
        for reducer in &reducers {
            state = reducer(state, &action).unwrap()
        }
        Ok(state)
    })
}
