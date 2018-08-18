use std::sync::mpsc;
use store::events::Event;
use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::app::{AppState, CommandHandler};

pub struct CommandMiddleWare {
    pub tx: mpsc::Sender<Event>,
    pub handler: CommandHandler
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
                                let cmd_fn = self.handler.cmd_reg[command.clone()];
                                self.tx.send(Event::CommandRun { 
                                    func: cmd_fn,
                                    uuid: uuid.to_string()
                                }).unwrap();
                                AppAction::CommandCreate(uuid.to_string()) 
                            },
                        };
                        let _ = store.dispatch(_action);
                    }
                    None => { debug!("Already Consumed {:?}", uuid); }
                }
            }
            _ => {}
        }
        next(store, action)
    }
}
