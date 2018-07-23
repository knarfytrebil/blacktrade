impl AppState {
    // helper functions
    fn set_mode(&mut self, mode: &str) {
        match mode {
            "command" => {
                self.mode = AppMode::get_mode("command");
                self.command.push(':');
            }
            "normal" => {
                self.mode = AppMode::get_mode("normal");
                self.command.clear();
            }
            _ => {}
        }
    }
}
