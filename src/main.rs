#[macro_use]
extern crate log;

// extern crate cpython;
extern crate handlebars;
extern crate redux;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate simplelog;
extern crate termion;
extern crate treexml;
extern crate tui;
extern crate unicode_width;
extern crate uuid;

#[macro_use]
mod utils;
mod actions;
mod components;
mod middlewares;
mod reducers;
mod structs;

use simplelog::*;
use std::boxed::Box;
use std::fs::File;
use std::io;
use std::sync::mpsc;

use structs::app::events::Event;

fn main() -> Result<(), io::Error> {
    // Init Logs
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    )])
    .unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    let (input_tx, subscribe_tx) = (cmd_tx.clone(), tx);

    let _ = utils::input::init(input_tx);
    let store = utils::store::init(&cmd_tx);

    // Create Subscription from store to render
    store.subscribe(Box::new(move |store, _| {
        let state = store.get_state();
        subscribe_tx.send(Event::Render(state)).expect("Send Error");
    }));

    utils::commands::connect(cmd_rx, store);
    utils::run::keep_alive(rx)
}
