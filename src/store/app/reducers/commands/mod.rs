use store::app::{AppState};
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
