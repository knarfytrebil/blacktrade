use std::sync::mpsc;
use store::events::Event;
use redux::{Store};
use store::loops::AppState;


fn writeConsole(line: String) {
    debug!("line: {:?}", line);
}

pub trait MainStore {
    fn get_writeConsole(tx: mpsc::Sender<Event>) -> fn(String);
}

impl MainStore for Store<AppState> {
     
    fn get_writeConsole(tx: mpsc::Sender<Event>) -> fn(String) { 
        writeConsole 
    }

}
