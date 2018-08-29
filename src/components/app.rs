pub mod instance {
    use std::io;
    // use std::io::{Error, ErrorKind};
    use structs::app::AppState;
    use tui::backend::MouseBackend;
    use tui::layout::{Direction, Group, Rect, Size};
    use tui::style::{Color, Style};
    use tui::widgets::{Tabs, Widget};
    use tui::Terminal;

    use components::command_bar;
    use components::command_output;
    use components::status_bar;

    // return Err(Error::new(ErrorKind::Interrupted, "Exit App"));
    pub fn render(terminal: &mut Terminal<MouseBackend>, app: &AppState) -> Result<(), io::Error> {
        let mut size = terminal.size().unwrap();

        if size != app.size && Rect::default() != app.size {
            size = app.size;
            terminal.resize(size).unwrap();
        }

        Group::default()
            .direction(Direction::Vertical)
            .sizes(&[Size::Fixed(1), Size::Min(1), Size::Fixed(1), Size::Fixed(1)])
            .render(terminal, &size, |t, chunks| {
                Tabs::default()
                    // .block(Block::default().borders(Borders::TOP))
                    .titles(&app.tabs.titles)
                    .style(Style::default().fg(Color::Green))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .select(app.tabs.selection)
                    .render(t, &chunks[0]);
                match app.tabs.selection {
                    0 => command_output::instance::render(t, app, chunks[1]),
                    1 => {}
                    _ => {}
                }
                status_bar::instance::render(t, app, chunks[2]);
                command_bar::instance::render(t, app, chunks[3]);
            });
        try!(terminal.draw());
        Ok(())
    }
}
