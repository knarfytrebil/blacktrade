extern crate log;
extern crate stderrlog;
extern crate termion; 
extern crate tui;
extern crate redux;

mod store;
mod components;

use std::io;
use std::io::{Write};
use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;

use redux::{Store};

use store::loops::App;
use components::application;

enum Event {
    Input(event::Key),
}

fn main() {
    stderrlog::new().verbosity(4).init().unwrap();

    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let mut _input_cmd = String::new();
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // App
    let mut app = App::new();
    // let app : Store<App> = Store::new(vec![]);

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    application::instance::render(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Char(':') => {
                    break;
                }
               _ => {}
            },
        }
        application::instance::render(&mut terminal, &app);
    }
    terminal.show_cursor().unwrap();
}
