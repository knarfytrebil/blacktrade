use std::sync::mpsc;
use std::collections::HashMap;
use reducers::{CommandGen, commands};
use structs::app::{Event};

// Experimental
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::{thread, panic};


pub struct CommandHandler {
    pub cmd_reg: HashMap<String, CommandGen>
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {
            cmd_reg: HashMap::new()
        }
    }

    pub fn default() -> Self {
        let mut handler = CommandHandler::new();
        handler.cmd_reg.insert("exec".to_string(), commands::helloworld);
        handler
    }
}

impl CommandHandler {
    pub fn spawn(&self, tx: mpsc::Sender<Event>, cmd_str: String) {
        let thread_tx = tx.clone();
        match thread::Builder::new()
            .name("test".to_string()).spawn(move || {
            // Panic Handler for Thread
            panic::set_hook(Box::new(|panic_info| {
                error!("A panic occurred: {:?}", &panic_info);
            }));
            let mut cmd_with_args: Vec<&str> = cmd_str.split(" ").collect();
            let command = cmd_with_args.remove(0);
            match Command::new(command)
                .args(cmd_with_args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn() {
                Ok(mut child) => {
                    let reader = child.stdout.take()
                        .expect("Couldn't get pipe stream");
                    let mut child_out = BufReader::new(reader);
                    loop {
                        let mut buffer = String::new();
                        let read_bytes = child_out.read_line(&mut buffer)
                            .expect("Unable to read bytes");
                        if read_bytes != 0 {
                            let _ = tx.send(Event::ConsolePush(buffer));
                        } 
                        else { break; }
                    }
                    let _ = tx.send(Event::ConsolePush("Command Finished\n".to_string()));
                }
                Err(error) => {
                    let err_str = format!("Panic: {:?}\n", error);
                    let _ = tx.send(Event::ConsolePush(err_str));
                }
            };
        }) {
            Ok(_result)=>{ let _ = thread_tx.send(Event::ConsolePush("Thread Successfully Spawned\n".to_string())); }
            Err(_)=>{ }
        }
    }
}
