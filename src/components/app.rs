pub mod instance {

    use std::io;
    use tui::Terminal;
    use tui::backend::MouseBackend;
    use tui::style::{Color, Style};
    use tui::layout::{Direction, Group, Size};
    use tui::widgets::{Block, Borders, Widget, Tabs};
    use store::loops::AppState;

    use components::status_bar;
    use components::command_bar;
    use components::command_output;

    pub fn render(t: &mut Terminal<MouseBackend>, app: &AppState) 
        -> Result<(), io::Error> {
        Group::default()
            .direction(Direction::Vertical)
            .sizes(&[Size::Fixed(3), Size::Min(1), Size::Fixed(1), Size::Fixed(1)])
            .render(t, &app.size, |t, chunks| {
                Tabs::default()
                    .block(Block::default().borders(Borders::ALL).title("Tabs"))
                    .titles(&app.tabs.titles)
                    .style(Style::default().fg(Color::Green))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .select(app.tabs.selection)
                    .render(t, &chunks[0]);
                    match app.tabs.selection {
                        0 => { command_output::instance::render(t, app, &chunks[1]) }
                        1 => { }
                        _ => { }
                    }
                status_bar::instance::render(t, app, &chunks[2]);
                command_bar::instance::render(t, app, &chunks[3]);
            });
        try!(t.draw());
        Ok(())
    }

}
