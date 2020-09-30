use components::command_bar;
use components::command_output;
use components::ele::powerline_tab::Tabs;
use components::status_bar;
use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::Frame;

pub fn render<B>(frame: &mut Frame<B>, store: &AppState)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let tabs = Tabs::default()
        .titles(&store.tabs.titles)
        .style(Style::default().fg(Color::Gray).bg(Color::Black))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .divider_style(Style::default().fg(Color::White).bg(Color::Black))
        .select(store.tabs.selection);

    frame.render_widget(tabs, chunks[0]);

    match store.tabs.selection {
        0 => command_output::render(frame, store, chunks[1]),
        1 => {}
        _ => {}
    }

    status_bar::render(frame, store, chunks[2]);
    command_bar::render(frame, store, chunks[3]);
}
