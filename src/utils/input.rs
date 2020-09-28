use std::{io, thread};
use std::sync::mpsc::{Sender};
use utils::app::to_serializable;
use termion::input::{TermRead};
use structs::app::events::Event;
use actions::AppAction;

pub fn init_keyboard_input(input_tx: Sender<Event>) {
    thread::spawn(move || {
        for c in io::stdin().keys() {
            let serializable = to_serializable(c.unwrap());
            let evt = AppAction::Keyboard(serializable).into_event();
            input_tx.send(evt).expect("Failed to Send");
        }
    });
}
