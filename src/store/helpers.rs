use redux::Store;
use std::sync::mpsc;
use store::app::AppState;
use store::events::Event;

pub fn write_console(line: String) {
    println!("{}", line);
}

pub trait MainStore {
    fn get_write_console(tx: mpsc::Sender<Event>) -> fn(String);
}

impl MainStore for Store<AppState> {
    fn get_write_console(tx: mpsc::Sender<Event>) -> fn(String) {
        match tx {
            _ => return |line| write_console(line),
        }
    }
}
