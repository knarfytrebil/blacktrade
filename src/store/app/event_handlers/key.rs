use store::app::mode::ModeCategory;
use store::app::AppState;
use termion::event;

impl AppState {
    pub fn key_event_handler(&mut self, evt: event::Key) {
        match self.mode.category {
            ModeCategory::Normal => {
                Self::normal_key_handler(self, evt);
            }
            ModeCategory::Command => {
                Self::command_key_handler(self, evt);
            }
        }
    }
}
