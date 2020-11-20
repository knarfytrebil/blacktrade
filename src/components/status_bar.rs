use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};

use tui::text::{Span, Spans};
use tui::widgets::Paragraph;
use tui::Frame;

pub fn render<B>(frame: &mut Frame<B>, store: &AppState, area: Rect)
where
    B: Backend,
{
    let value = store.json_store["mode"]["symbol"]
        .as_str()
        .expect("JSON Error");
    let text = Spans::from(vec![
        Span::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(value, Style::default().bg(Color::Black)),
        Span::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(
            "\u{E0B0} ",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Span::styled(
            "Runing Tasks [0]",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Span::styled(" ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::styled(
            "\u{E0B0}",
            Style::default().fg(Color::White).bg(Color::Black),
        ),
    ]);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, area);
}
