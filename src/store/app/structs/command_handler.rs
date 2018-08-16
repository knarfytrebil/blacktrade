use store::app::AppState;

pub type CmdCallback = fn(&mut AppState, String) -> bool;

impl AppState {
    pub fn exit(&mut self, txt: String) -> bool {
        true
    }
}
