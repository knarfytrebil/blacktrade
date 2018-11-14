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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), 
            Constraint::Min(1), 
            Constraint::Length(1), Constraint::Length(1)].as_ref())
        .split(app.size);

        Tabs::default()
            // .block(Block::default().borders(Borders::TOP))
            .titles(&app.tabs.titles)
            .style(Style::default().fg(Color::Green))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(app.tabs.selection)
            .render(frame, chunks[0]);

        match app.tabs.selection {
            0 => command_output::render(frame, app, chunks[1]),
            1 => {}
            _ => {}
        }

        status_bar::render(frame, app, chunks[2]);
        command_bar::render(frame, app, chunks[3]);
}
