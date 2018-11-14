use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::{Direction, Layout, Constraint};
use tui::style::{Color, Style};
use tui::widgets::{Tabs, Widget};
use tui::Frame; 
use components::command_bar;
use components::command_output;
use components::status_bar;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState) 
where 
    B: Backend
{
//    let mut size = terminal.size().unwrap();
//
//    if size != app.size && Rect::default() != app.size {
//        size = app.size;
//        terminal.resize(size).unwrap();
//    }

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(&[
            Constraint::Fixed(1), 
            Constraint::Min(1), 
            Constraint::Fixed(1), Constraint::Fixed(1)])
        .render(&mut frame, |t, chunks| {
            Tabs::default()
                // .block(Block::default().borders(Borders::TOP))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
            match app.tabs.selection {
                0 => command_output::render(t, app, chunks[1]),
                1 => {}
                _ => {}
            }
            status_bar::render(t, app, chunks[2]);
            command_bar::render(t, app, chunks[3]);
        });
}
