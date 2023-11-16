use components::command_bar;
use components::command_output;
use components::ele::powerline_tab::Tabs;
use components::status_bar;
use components::xml;
use structs::app::AppState;
use serde_json::Value;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::Frame;

pub fn p_render(
    frame: &mut Frame, 
    store: &AppState, 
    area: Rect, 
    template: fn() -> String,
    props: fn(&Value, Rect)-> Value
) {
    let dom_root = xml::parse(
        template(),
        &props(&store.json_store, area),
    );

    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !"),
    };

    frame.render_widget(widget, area);
}

pub fn render(frame: &mut Frame, store: &AppState)
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

    match store.tabs.selection {
        0 => p_render(frame, store, chunks[1], command_output::template, command_output::props),
        1 => {}
        _ => {}
    }

    p_render(frame, store, chunks[2], status_bar::template, status_bar::props);
    p_render(frame, store, chunks[3], command_bar::template, command_bar::props);

    frame.render_widget(tabs, chunks[0]);

}
