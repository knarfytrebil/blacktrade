use std::collections::HashMap;

type Callback = fn(String) -> Option<()>;

#[derive(Clone, Debug)]
pub struct CommandHandler {
    command_function: HashMap<String, Callback>,
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {
            command_function: HashMap::new()
        }
    }

    fn add_command_function(&mut self, name: String, func: Callback) {
        self.command_function.insert(name, func);
    }
}
