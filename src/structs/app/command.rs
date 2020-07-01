use structs::app::AppState;

pub type CmdCallback = fn(&mut AppState, String) -> bool;

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub id: String,
    pub failed: bool
}

impl Command {
    pub fn new(name: String, id: String, failed: bool) -> Command {
        Command { name: name, id: id, failed: failed }
    }
}
