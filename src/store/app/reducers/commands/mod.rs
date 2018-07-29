use store::app::AppState;

impl AppState {
    pub fn command_handler(&mut self, cmd: String) {
        debug!("got command {:?}", &cmd);
    }
}
