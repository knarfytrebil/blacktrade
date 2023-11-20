use std::io;
use std::sync::mpsc::Receiver;

use structs::app::events::Event;
use components::app;
use components::helpers::{hb_macros, hb_utils};
use components::helpers::height_buffer::HEIGHT_BUFFER_HELPER;

use termion::{
    input::MouseTerminal,
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};
use handlebars::Handlebars;

use ratatui::backend::TermionBackend;
use ratatui::Terminal;

pub fn keep_alive(receiver: Receiver<Event>) -> Result<(), io::Error> {
    // Terminal initialization

    let stdout = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();
    terminal.hide_cursor()?;
    let mut reg = Handlebars::new();
    reg.register_helper("stringify", Box::new(hb_macros::stringify));
    reg.register_helper("powerline_symbol", Box::new(hb_macros::powerline_symbol));
    reg.register_helper("height_buffer", Box::new(HEIGHT_BUFFER_HELPER));
    reg.register_escape_fn(hb_utils::escape_nothing);

    loop {
        match receiver.recv().unwrap() {
            Event::Render(app_state) => { 
                _ = terminal.draw(|mut f| {
                    app::render(
                        &mut f, 
                        &app_state,
                        reg.clone()
                    )
                });
            }
            Event::Exit => { break; }
            _ =>  { break; }
        }
    }

    // show cursor on end
    terminal.show_cursor()?;
    Ok(())
}
