use cpython::Python;
use store::app::structs::get_quotes;
use store::app::utils::{get_snippet, python};
use store::app::AppState;
use termion::event;

impl AppState {
    pub fn command_key_handler(&mut self, evt: event::Key) {
        match evt {
            event::Key::Esc => {
                self.set_mode("normal");
            }
            event::Key::Backspace => {
                if self.command == ":" {
                    self.set_mode("normal");
                } else {
                    self.command.pop();
                }
            }
            // Must be above Char(_char)
            event::Key::Char('\n') => {
                let cmd = self.command.split_off(1);
                match cmd.as_str() {
                    "q" => {
                        self.exiting = true;
                    }
                    _ => {
                        info!("Command Issued: {:?}", cmd);
                        let line = format_output!("green", "command", &cmd);
                        self.console_txt.push_str(&line);
                        let gil = Python::acquire_gil();
                        let py = gil.python();
                        let quotes = get_quotes();
                        let code = get_snippet();
                        let res = python::run(py, &quotes, code);
                        self.console_txt
                            .push_str(&format_output!("yellow", "pyout", &res));
                    }
                }
            }
            event::Key::Char(_char) => {
                self.command.push(_char);
            }
            _ => {}
        }
    }
}
