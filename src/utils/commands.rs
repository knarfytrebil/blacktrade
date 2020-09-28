use std::{thread};
use structs::app::{AppState};
use std::sync::mpsc::{Receiver};
use std::sync::{Arc};
use redux::Store;
use structs::app::events::Event;


pub fn connect(
    receiver: Receiver<Event>,
    store: Arc<Store<AppState>>
) {
    thread::spawn(move || loop {
        match receiver.recv().unwrap() {
            Event::Dispatch(action) => {
                debug!("ACTION DISPATCHED {:?}", &action);
                let _ = store.dispatch(action);
            }
            //Event::Exit => {
            //    let _ = exit_tx.send(Event::Exit);
            //    break;
            //}
            _ => {}
        }
    });
}

