use std::sync::mpsc;
use actions::AppAction;
use redux::{DispatchFunc, Middleware, Store};
use structs::app::{AppState, CommandHandler};
use structs::app::events;

pub struct CommandMiddleWare {
    pub tx: mpsc::Sender<events::Event>,
    pub handler: CommandHandler
}

impl Middleware<AppState> for CommandMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        match &action {
            &AppAction::CommandBarEnqueueCmd(ref uuid) => { 
                let evt = AppAction::CommandConsume(uuid.to_string()).to_event();
                self.tx.send(evt).unwrap(); 
            }
            &AppAction::CommandConsume(ref uuid) => {
                let state = store.get_state();
                match state.cmd_str_queue.get(uuid) {
                    Some(command) => {
                        let mut cmd_with_args: Vec<&str> = command.split(" ").collect();
                        let cmd_str = cmd_with_args.remove(0);
                        let _action = match self.handler.cmd_reg.contains_key(cmd_str) {
                            false => { AppAction::CommandInvalid(uuid.to_string()) }
                            true => { 
                                self.handler.spawn(self.tx.clone(), cmd_with_args.join(" "), uuid.to_string());
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

