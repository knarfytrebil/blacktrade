use redux::Store;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use structs::app::events::Event;
use structs::app::{AppState, CommandHandler};

use middlewares::{
    CommandBarMiddleWare, CommandMiddleWare, ConsoleMiddleWare, DebugMiddleWare, KeyboardMiddleWare,
};

pub fn init(cmd_tx: &Sender<Event>) -> Arc<Store<AppState>> {
    let keyboard_mw = Box::new(KeyboardMiddleWare {});
    let command_bar_mw = Box::new(CommandBarMiddleWare {});
    let command_mw = Box::new(CommandMiddleWare {
        tx: cmd_tx.clone(),
        handler: CommandHandler::default(),
    });
    let console_mw = Box::new(ConsoleMiddleWare {});
    let debug_mw = Box::new(DebugMiddleWare {});

    Arc::new(Store::new(vec![
        console_mw,
        command_bar_mw,
        command_mw,
        keyboard_mw,
        debug_mw,
        // exit_mw,
    ]))
}
