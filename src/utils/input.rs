use actions::AppAction;
use std::sync::mpsc::Sender;
use std::{io, thread};
use structs::app::events::Event;
use termion::input::TermRead;
use utils::app::to_serializable;

pub fn init(input_tx: Sender<Event>) {
    thread::spawn(move || {
        for c in io::stdin().keys() {
            let serializable = to_serializable(c.unwrap());
            let evt = AppAction::Keyboard(serializable).into_event();
            input_tx.send(evt).expect("Failed to Send");
        }
    });
}