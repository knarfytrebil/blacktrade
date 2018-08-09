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

pub fn create() -> Box<ReducerFn> {
    Box::new(|mut state: AppState, action: &AppAction| -> Result<AppState, String> {
        match action {
            AppAction::CommandCreate(uuid) => {
                let command = state.cmd_str_queue[uuid].clone();
                state.cmd_running.push(Command::new(command,uuid.clone()));
                Ok(state)
            }
            _ => { Ok(state) }
        }
    })
}
