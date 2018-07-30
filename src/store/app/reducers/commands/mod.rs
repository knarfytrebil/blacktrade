use store::app::AppState;

impl AppState {
    pub fn command_handler(&mut self, cmd: String) -> Option<()> {
        debug!("got command {:?}", &cmd);
        match self.cmd_reg.contains_key(&cmd) {
            true => {
                self.cmd_reg[&cmd](self, "test".to_string());
                return Some(());
            }
            false => {
                return None;
            }
        }
    }
}
