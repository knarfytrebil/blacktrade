pub mod instance {
    use std::io;
    use std::io::{Error, ErrorKind};
    use store::loops::AppState;
    use tui::backend::MouseBackend;
    use tui::layout::{Direction, Group, Size};
    use tui::style::{Color, Style};
    use tui::widgets::{Tabs, Widget};
    use tui::Terminal;

    use components::command_bar;
    use components::command_output;
    use components::status_bar;

    pub fn render(terminal: &mut Terminal<MouseBackend>, app: &AppState) -> Result<(), io::Error> {
        if app.exiting {
            return Err(Error::new(ErrorKind::Interrupted, "Exit App"));
        }
        Group::default()
            .direction(Direction::Vertical)
            .sizes(&[Size::Fixed(1), Size::Min(1), Size::Fixed(1), Size::Fixed(1)])
            .render(terminal, &app.size, |t, chunks| {
                Tabs::default()
                    // .block(Block::default().borders(Borders::TOP))
                    .titles(&app.tabs.titles)
                    .style(Style::default().fg(Color::Green))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .select(app.tabs.selection)
                    .render(t, &chunks[0]);
                match app.tabs.selection {
                    0 => command_output::instance::render(t, app, &chunks[1]),
                    1 => {}
                    _ => {}
                }
                status_bar::instance::render(t, app, &chunks[2]);
                command_bar::instance::render(t, app, &chunks[3]);
            });
        try!(terminal.draw());
        return Ok(());
    }
}
