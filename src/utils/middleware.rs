use structs::app::{AppState, CommandHandler};
use std::sync::mpsc::{Sender};
use structs::app::events::Event;
use std::sync::{Arc};
use redux::Store;

use middlewares::{
    CommandBarMiddleWare, 
    CommandMiddleWare, 
    ConsoleMiddleWare, 
    DebugMiddleWare, 
    KeyboardMiddleWare,
};

pub fn init_store(cmd_tx: &Sender<Event>) -> Arc<Store<AppState>> {
    let keyboard_mw = Box::new(KeyboardMiddleWare {});
    let command_bar_mw = Box::new(CommandBarMiddleWare {});
    let command_mw = Box::new(CommandMiddleWare {
        tx: cmd_tx.clone(),
        handler: CommandHandler::default(),
    });
    let console_mw = Box::new(ConsoleMiddleWare {});
    let debug_mw = Box::new(DebugMiddleWare {});

    // let (exit_tx, _exit_rx) = mpsc::channel();
    // let exit_mw = Box::new(CommandMiddleWare {
    //     tx: exit_tx,
    //     handler: CommandHandler::default(),
    // });

    Arc::new(Store::new(vec![
        console_mw,
        command_bar_mw,
        command_mw,
        keyboard_mw,
        debug_mw,
        // exit_mw,
    ]))
}

