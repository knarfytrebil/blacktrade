use std::sync::mpsc;
use store::events::Event;
use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::{AppState, CommandHandler};

// Experimental
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::thread;

pub struct CommandMiddleWare {
    pub tx: mpsc::Sender<Event>,
    pub handler: CommandHandler
}

impl CommandHandler {
    fn spawn(&self, tx: mpsc::Sender<Event>)  {
        thread::spawn(move || {
            let command = "/bin/bash";
            let mut child = Command::new(command)
                .arg("./src/scripts/spot.sh")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to Start");

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

            let _  = tx.send(Event::ConsolePush("BreakLoop".to_string()));
            
        });
    }
}

impl Middleware<AppState> for CommandMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        // debug!("5 {:?}", &action);
        match &action {
            &AppAction::CommandBarEnqueueCmd(ref uuid) => { self.tx.send(Event::CommandQueued(uuid.to_string())).unwrap(); }
            &AppAction::CommandConsume(ref uuid) => {
                let state = store.get_state();
                match &state.cmd_str_queue.get(uuid) {
                    Some(command) => {
                        let _action = match self.handler.cmd_reg.contains_key(command.clone()) {
                            false => { AppAction::CommandInvalid(uuid.to_string()) }
                            true => { 
                                self.handler.spawn(self.tx.clone());
                                AppAction::CommandCreate(uuid.to_string()) 
                            },
                        };
                        let _ = store.dispatch(_action);
                    }
                    None => { debug!("No Command in Queue{:?}", uuid); }
                }
            }
            _ => {}
        }
        next(store, action)
    }
}

// self.tx.send(Event::CommandRun { 
//     func: self.handler.cmd_reg[command.clone()],
//     uuid: uuid.to_string()
// }).unwrap();

