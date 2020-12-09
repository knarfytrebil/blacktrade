use actions::AppAction;
use reducers::{commands, CommandGen};
use std::collections::HashMap;
use std::sync::mpsc;
use structs::app::events;

// Experimental
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{panic, thread};

pub struct CommandHandler {
    pub cmd_reg: HashMap<String, CommandGen>,
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {
            cmd_reg: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        let mut handler = CommandHandler::new();
        handler
            .cmd_reg
            .insert("exec".to_string(), commands::do_nothing);
        handler
            .cmd_reg
            .insert("q".to_string(), commands::do_nothing);
        handler
    }
}

impl CommandHandler {
    pub fn spawn(&self, tx: mpsc::Sender<events::Event>, cmd_str: String, uuid: String) {
        debug!( "CMD STR {:?} ", &cmd_str);
        let thread_tx = tx.clone();
        let res = match thread::Builder::new().name(uuid.clone()).spawn(move || {
            // Panic Handler for Thread
            panic::set_hook(Box::new(|panic_info| {
                error!("A panic occurred: {:?}", &panic_info);
            }));
            let mut cmd_with_args: Vec<&str> = cmd_str.split(' ').collect();
            let command = cmd_with_args.remove(0);
            let res_action = match Command::new(command)
                .args(cmd_with_args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(mut child) => {
                    let reader = child.stdout.take().expect("Couldn't get pipe stream");
                    let mut child_out = BufReader::new(reader);
                    loop {
                        let mut buffer = String::new();
                        let read_bytes = child_out
                            .read_line(&mut buffer)
                            .expect("Unable to read bytes");
                        if read_bytes != 0 {
                            // if &buffer != "1" {
                            //     let _ = tx.send(events::Event::Exit);
                            // }
                            let evt = AppAction::ConsolePush(buffer).into_event();
                            let _ = tx.send(evt);
                        } else {
                            break;
                        }
                    }
                    AppAction::CommandEnd {
                        uuid: uuid.clone(),
                        success: true,
                        reason: String::new(),
                    }
                }
                Err(error) => {
                    let err_str = format!("Child Panic: {:?}", error);
                    AppAction::CommandEnd {
                        uuid: uuid.clone(),
                        success: false,
                        reason: err_str,
                    }
                }
            };
            let _ = tx.send(res_action.into_event());
        }) {
            Ok(_result) => format_output!("green", "...", "Thread Spawned"),
            Err(_) => format_output!("red", "!!!", "Thread Failed"),
        };
        let evt = AppAction::ConsolePush(res).into_event();
        let _ = thread_tx.send(evt);
    }
}
