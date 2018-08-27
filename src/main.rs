#[macro_use]
extern crate log;

extern crate uuid;
extern crate cpython;
extern crate redux;
extern crate simplelog;
extern crate termion;
extern crate tui;
extern crate regex;

#[macro_use]
mod utils;
mod components;
mod middlewares;
mod structs;
mod actions;
mod reducers;

use simplelog::*;
use std::boxed::Box;
use std::fs::File;
use std::sync::mpsc;
use std::{io, process, thread};

use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::Terminal;
use redux::Store;

use middlewares::*;
use actions::AppAction;
use structs::app::{AppState, CommandHandler};
use structs::app::events::Event;
use components::app;

fn main() {
    // Init Logs
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    )]).unwrap();


    // Channels
    let (tx, rx) = mpsc::channel();
    let (cmd_tx, cmd_rx): (mpsc::Sender<Event>, mpsc::Receiver<Event>) = mpsc::channel();

    let (input_tx, subscribe_tx) = (cmd_tx.clone(), tx.clone());

    // Input
    thread::spawn(move || {
        for c in io::stdin().keys() {
            input_tx.send(
                AppAction::Keyboard(c.unwrap()).to_event()
            ).unwrap();
        }
    });

    let cmd_handler = CommandHandler::default();

    // Middlewares
    let keyboard_mw = Box::new(KeyboardMiddleWare { });
    let command_bar_mw = Box::new(CommandBarMiddleWare { });
    let command_mw = Box::new(CommandMiddleWare { tx: cmd_tx, handler: cmd_handler });
    let console_mw = Box::new(ConsoleMiddleWare { });
    let debug_mw = Box::new(DebugMiddleWare { });

    // App & State
    let store: Store<AppState> = Store::new(vec![
        command_mw,
        console_mw,
        command_bar_mw,
        keyboard_mw,
        debug_mw,
    ]);

    // Create Subscription from store to render
    store.subscribe(Box::new(move |store, _| {
        let state = store.get_state();
        subscribe_tx.send(Event::Render(state)).unwrap();
    }));


    thread::spawn(move || {
        loop {
            match cmd_rx.recv().unwrap() {
                Event::Dispatch(action) => { let _ = store.dispatch(action); },
                _ => {}
            }
        }
    });

    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    let mut app_size = terminal.size().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app_size {
            terminal.resize(size).unwrap();
            app_size = size;
        }

        match rx.recv().unwrap() {
            Event::Render(app_state) => { app::instance::render(&mut terminal, &app_state, app_size).unwrap(); }
            Event::Exit => { break; }
            _ => {}
        }
    }

    // show cursor on end
    terminal.show_cursor().unwrap();
    process::exit(1);
}
