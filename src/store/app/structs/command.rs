
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    name: String,
    pub id: String,
    failed: bool
}

impl Command {
    pub fn new(name: String, id: String) -> Command {
        Command {
            name: name,
            id: id,
            failed: false
        }
    }
}
