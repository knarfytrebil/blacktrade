extern crate redux;
extern crate termion;
extern crate tui;

#[macro_use]
extern crate log;
extern crate simplelog;

mod components;
mod middlewares;
mod store;
mod utils;

use simplelog::*;
use std::boxed::Box;
use std::fs::File;
use std::io::ErrorKind;
use std::sync::mpsc;
use std::{io, process, thread};

use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::Terminal;

use components::app;
use middlewares::term::Term;
use redux::Store;
use store::events::Event;
use store::loops::{AppAction, AppState};

fn main() {
    // Init Logs
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    )]).unwrap();

    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();

    let (input_tx, render_tx, term_tx) = (tx.clone(), tx.clone(), tx.clone());

    // Input
    thread::spawn(move || {
        for c in io::stdin().keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
        }
    });

    // Middlewares
    let term_mw = Box::new(Term { tx: term_tx });

    // App & State
    let store: Store<AppState> = Store::new(vec![term_mw]);

    // Create Subscription from store to render
    store.subscribe(Box::new(move |store, _| {
        render_tx.send(Event::Render(store.get_state())).unwrap();
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

        match rx.recv().unwrap() {
            Event::Input(input) => {
                let _ = store.dispatch(AppAction::Keyboard(input));
            }
            Event::Render(app_state) => {
                match app::instance::render(&mut terminal, &app_state) {
                    Err(e) => match e.kind() {
                        ErrorKind::Interrupted => {
                            break;
                        }
                        _ => {
                            eprintln!("Application Error: {}", e);
                            process::exit(1);
                        }
                    },
                    Ok(_) => {}
                };
            }
        }
    }

    // show cursor on end
    terminal.show_cursor().unwrap();
}
