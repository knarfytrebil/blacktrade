use redux::Store;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use structs::app::events::Event;
use structs::app::AppState;

pub fn connect(receiver: Receiver<Event>, store: Arc<Store<AppState>>, exit_tx: Sender<Event>) {
    thread::spawn(move || loop {
        match receiver.recv().unwrap() {
            Event::Dispatch(action) => {
                debug!("command action dispatched {:?}", &action);
                let _ = store.dispatch(action);
            }
            Event::Exit => {
                let _ = exit_tx.send(Event::Exit);
                break;
            }
            _ => {}
        }
    });
}
