extern crate stderrlog;
extern crate termion; 
extern crate tui;
extern crate redux;

mod store;
mod components;
mod utils;
mod middlewares;

use std::io;
use std::thread;
use std::sync::mpsc;
use std::boxed::Box;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;

use redux::{Store};

use store::loops::{AppState, AppAction};
use middlewares::term::Term;
use components::app;

enum Event {
    Input(event::Key),
    Render(AppState),
}

fn main() {
    stderrlog::new().verbosity(4).init().unwrap(); 
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let render_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // App & State
    let store:Store<AppState> = Store::new(vec![]);

    // Create Subscription from store to render
    store.subscribe(Box::new(move |store, _| {
        render_tx.send(
            Event::Render(store.get_state())
        ).unwrap();
    }));

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let size = terminal.size().unwrap();
    let _ = store.dispatch(AppAction::ResizeApp(size));

    loop {
        let size = terminal.size().unwrap();
        let app_state = store.get_state();

        if size != app_state.size {
            terminal.resize(size).unwrap();
            let _ = store.dispatch(AppAction::ResizeApp(size));
        }
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input { 
                event::Key::Char('q') => { break; },
                _ => { store.dispatch(AppAction::Keyboard(input)); }
            },
            Event::Render(app_state) => { 
                app::instance::render(&mut terminal, &app_state); 
            },
            _ => {}
        }
    }

    // show cursor on end
    terminal.show_cursor().unwrap();
}
