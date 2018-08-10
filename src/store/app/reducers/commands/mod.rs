use store::app::{AppState, Command};
use store::action::AppAction;
use store::app::reducers::ReducerFn;

impl AppState {
    pub fn command_handler(&mut self, cmd: String) -> Option<()> {
        debug!("got command {:?}", &cmd);
        self.cmd_reg[&cmd](self, "test".to_string());
        return Some(());
    }
}


pub fn verify() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandBarSet(str_ref) => {
                state.command = str_ref.to_string();
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}

pub fn create(failed: bool) -> Box<ReducerFn> {
    Box::new(move |mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandCreate(uuid) | AppAction::CommandInvalid(uuid) => {
                let cmd_str = state.cmd_str_queue[uuid].clone();
                let cmd_obj = Command::new(cmd_str, uuid.clone(), failed);
                match &failed {
                    true => { state.cmd_ended.push(cmd_obj) }
                    false => { state.cmd_running.push(cmd_obj) }
                }
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
