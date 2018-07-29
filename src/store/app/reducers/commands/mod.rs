use store::app::ModeCategory;
use store::app::AppState;
use termion::event;

impl AppState {
    pub fn command_handler(&mut self, cmd: String) {
        debug!("got command {:?}", &cmd);
    }
}
