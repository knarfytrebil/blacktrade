use store::app::AppState;

impl AppState {
    pub fn error_handler(&mut self, error: String) {
        self.console_txt.push_str(&error);
    }
}
