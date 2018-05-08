extern crate stderrlog;
extern crate termion; 
extern crate tui;
extern crate redux;

mod store;
mod components;

use std::io;
use std::thread;
use std::sync::mpsc;
use std::boxed::Box;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;

use redux::{Store};

use store::loops::{App, AppAction};
use components::application;

enum Event {
    Input(event::Key),
    Render(App<'static>), // <- FIXME: Probably wrong lifetime here
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

    // App & State
    let store:Store<App> = Store::new(vec![]);

    // Create Subscription from store to render
    store.subscribe(Box::new(move |store, _| {
        let app = store.get_state();
        render_tx.send(Event::Render(app)).unwrap();
    }));

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let size = terminal.size().unwrap();
    let _ = store.dispatch(AppAction::ResizeApp(size));


    loop {
        let size = terminal.size().unwrap();
        let app = store.get_state();

        if size != app.size {
            terminal.resize(size).unwrap();
            let _ = store.dispatch(AppAction::ResizeApp(size));
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
            Event::Render(app) => match app {  
                _ => { 
                    application::instance::render(&mut terminal, &app);
                }
            }
        }
    }
     
    // show cursor on end
    terminal.show_cursor().unwrap();

}
