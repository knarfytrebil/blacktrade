use std::sync::mpsc::{Receiver};
use std::{io};
use structs::app::events::Event;
use components::app;

use termion::input::{MouseTerminal};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::backend::TermionBackend;
use tui::Terminal;

pub fn until_break(
    receiver: Receiver<Event>,
) -> Result<(), io::Error> {

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();
    terminal.hide_cursor()?;

    loop {
        let _ = match receiver.recv().unwrap() {
            Event::Render(app_state) => terminal.draw(|mut f| app::render(&mut f, &app_state)),
            Event::Exit => {
                break;
            }
            _ => Ok(()),
        };
    }

    // show cursor on end
    terminal.show_cursor()?;
    Ok(())
}
