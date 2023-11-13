use components::app;
use std::io;
use std::sync::mpsc::Receiver;
use structs::app::events::Event;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use ratatui::backend::TermionBackend;
use ratatui::Terminal;

pub fn keep_alive(receiver: Receiver<Event>) -> Result<(), io::Error> {
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
            Event::Render(app_state) => { 
                terminal.draw(|mut f| app::render(&mut f, &app_state));
            }
            Event::Exit => {
                break;
            }
            _ =>  {
                break;
            }
        };
    }

    // show cursor on end
    terminal.show_cursor()?;
    Ok(())
}
