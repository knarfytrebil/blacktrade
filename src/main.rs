extern crate termion; 
extern crate tui;
extern crate redux;
extern crate logwatcher;

#[macro_use]
extern crate log;
extern crate simplelog;

mod store;
mod components;
mod utils;
mod middlewares;

use std::io;
use std::thread;
use std::sync::mpsc;
use std::boxed::Box;
use std::fs::File;

use logwatcher::LogWatcher;
use simplelog::*;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;

use redux::{Store};
use store::helpers::MainStore;

use store::loops::{AppState, AppAction};
use store::events::Event;
use middlewares::term::Term;
use components::app;

fn main() {
    // Init Logs
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug, 
            Config::default(), 
            File::create("debug.log")
            .unwrap())
    ]).unwrap();

    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let (input_tx, render_tx, term_tx, console_tx) 
        = (tx.clone(), tx.clone(), tx.clone(), tx.clone());

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

    // Middlewares
    let term_mw = Box::new(Term{ tx: term_tx }); 
            
    // App & State
    let store:Store<AppState> = Store::new(vec![term_mw]);

    //watcher
    let mut log_watcher = LogWatcher::register("debug.log".to_string()).unwrap();
    thread::spawn(move || {
        let f = MainStore::get_writeConsole(console_tx);
        log_watcher.watch(f);
    });

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
        match rx.recv().unwrap() {
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
