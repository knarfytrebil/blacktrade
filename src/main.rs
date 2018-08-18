#[macro_use]
extern crate log;

extern crate uuid;
extern crate cpython;
extern crate redux;
extern crate simplelog;
extern crate termion;
extern crate tui;

#[macro_use]
mod utils;
mod components;
mod middlewares;
mod store;

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
use store::action::AppAction;
use store::app::{AppState, CommandHandler};
use store::events::Event;
use components::app;

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

    let (input_tx, subscribe_tx) = (tx.clone(), tx.clone());

    // Input
    thread::spawn(move || {
        for c in io::stdin().keys() {
            input_tx.send(Event::Input(c.unwrap())).unwrap();
        }
    });

    let cmd_handler =  CommandHandler::default();

    // Middlewares
    let keyboard_mw = Box::new(KeyboardMiddleWare { });
    let command_bar_mw = Box::new(CommandBarMiddleWare { });
    let command_mw = Box::new(CommandMiddleWare { tx: tx.clone(), handler: cmd_handler });
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
        subscribe_tx.send(Event::Render(store.get_state())).unwrap();
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
            Event::CommandQueued(uuid_str) => { let _ = store.dispatch(AppAction::CommandConsume(uuid_str)); }
            Event::CommandRun { func, uuid } => { let _ = store.dispatch(AppAction::CommandRun{ func: func, uuid: uuid }); }
            Event::Input(input) => { let _ = store.dispatch(AppAction::Keyboard(input)); }
            Event::Render(app_state) => { app::instance::render(&mut terminal, &app_state).unwrap(); }
            Event::Exit => { break; }
        }
    }

    // process::exit(1);
    // show cursor on end
    terminal.show_cursor().unwrap();
}
