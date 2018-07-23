use termion::event;
use store::app::{AppState};

impl AppState {
    pub fn normal_key_handler(&mut self, evt: event::Key) {
        match evt {
            event::Key::Char(':') => {
                self.set_mode("command");
            }
            _ => {
                info!("unimplemented");
            }
        }
    }
}
