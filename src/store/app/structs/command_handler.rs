use store::app::AppState;

pub type CmdCallback = fn(&mut AppState, String) -> bool;

impl AppState {
    pub fn add_command_function(&mut self, name: String, func: CmdCallback) {
        self.cmd_reg.insert(name, func);
    }

    pub fn test_cb(&mut self, txt: String) -> bool {
        debug!("hello, {:?}", txt);
        self.exiting = true;
        true
    }
}
