use actions::AppAction;
use redux::{DispatchFunc, Middleware, Store};
use std::sync::mpsc;
use structs::app::events;
use structs::app::{AppState, CommandHandler};

pub struct CommandMiddleWare {
    pub tx: mpsc::Sender<events::Event>,
    pub handler: CommandHandler,
}

impl Middleware<AppState> for CommandMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("5 {:?}", &action);
        match action {
            AppAction::CommandBarEnqueueCmd(ref uuid) => {
                let evt = AppAction::CommandConsume(uuid.to_string()).into_event();
                self.tx.send(evt).expect("Failed to Send");
            }
            AppAction::CommandConsume(ref uuid) => {
                let state = store.get_state();
                match state.cmd_str_queue.get(uuid) {
                    Some(command) => {
                        let mut cmd_with_args: Vec<&str> = command.split(' ').collect();
                        let cmd_str = cmd_with_args.remove(0);
                        // if command registry contains "exec"
                        // currently the only key
                        debug!("CMD STR {:?}", cmd_str);
                        let _action = match self.handler.cmd_reg.contains_key(cmd_str) {
                            true => match cmd_str {
                                "exec" => {
                                    debug!("EXIT SIGNAL SENT 1");
                                    self.handler.spawn(
                                        self.tx.clone(),
                                        cmd_with_args.join(" "),
                                        uuid.to_string(),
                                    );
                                    AppAction::CommandCreate(uuid.to_string())
                                }
                                "q" => {
                                    self.tx.send(events::Event::Exit).expect("Failed to Send");
                                    AppAction::ConsolePush("Exiting...".to_string())
                                }
                                &_ => AppAction::CommandInvalid(uuid.to_string()),
                            },
                            false => AppAction::CommandInvalid(uuid.to_string()),
                        };
                        let _ = store.dispatch(_action);
                    }
                    None => {
                        debug!("No Command in Queue{:?}", uuid);
                    }
                }
            }
            _ => {}
        }
        next(store, action)
    }
}