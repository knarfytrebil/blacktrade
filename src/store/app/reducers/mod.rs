mod commands; mod keyboard;
mod error; mod mode;
mod size; mod command_bar;
mod console;

use redux::Reducer;
use store::action::AppAction;
use store::app::AppState;

type ReducerFn = Fn(AppState, &AppAction) -> Result<AppState, String>;
type ReducerArray = Vec<Box<ReducerFn>>;

impl Reducer for AppState {
    type Action = AppAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        debug!("[PreReduce]: {:?}", &action);
        let reducers: ReducerArray = match &action {
            &AppAction::ResizeApp(_) => { vec![size::set()] }
            &AppAction::SetMode(_) => { vec![mode::set()] }
            &AppAction::CommandBarPush(_) => { vec![command_bar::push()] }
            &AppAction::CommandBarSet(_) => { vec![command_bar::set()] }
            &AppAction::CommandBarEnqueueCmd(_) => { vec![command_bar::enqueue_cmd()] }
            &AppAction::ConsolePush(_) => { vec![console::push()] }
            &AppAction::CommandCreate(_) => { vec![commands::create(false)] }
            &AppAction::CommandInvalid(_) => { vec![commands::create(true)] }
            // AppAction::Keyboard(key_evt) => {
            //     Self::key_event_handler(self, key_evt);
            // }
            // AppAction::Command(Phase::Run(cmd)) => {
            //     Self::command_handler(self, cmd);
            // }
            // AppAction::Error(error) => {
            //     Self::error_handler(self, error);
            // }
            _ => { vec![] }
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
