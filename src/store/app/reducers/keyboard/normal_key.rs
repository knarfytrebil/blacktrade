use store::app::AppState;
use termion::event;

impl AppState {
    pub fn normal_key_handler(&mut self, evt: event::Key) {
        // match evt {
        //     event::Key::Char(':') => {
        //         self.set_mode("command");
        //     }
        //     _ => {
        //         info!("unimplemented");
        //     }
        // }
    }
}
