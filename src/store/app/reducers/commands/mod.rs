use store::app::AppState;

impl AppState {
    pub fn command_handler(&mut self, cmd: String) -> Option<()> {
        debug!("got command {:?}", &cmd);
        self.cmd_reg[&cmd](self, "test".to_string());
        return Some(());
    }
}
