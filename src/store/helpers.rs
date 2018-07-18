use redux::Store;
use std::sync::mpsc;
use store::events::Event;
use store::loops::AppState;

pub fn writeConsole(line: String) {}

pub trait MainStore {
    fn get_writeConsole(tx: mpsc::Sender<Event>) -> fn(String);
}

impl MainStore for Store<AppState> {
    fn get_writeConsole(tx: mpsc::Sender<Event>) -> fn(String) {
        writeConsole
    }
}
