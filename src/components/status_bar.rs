use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};

use tui::widgets::{Paragraph, Text};
use tui::Frame;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
        Text::styled(app.mode.symbol.clone(), Style::default().bg(Color::Black)),
        Text::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
        Text::styled(
            "\u{E0B0} ",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Text::styled(
            "Runing Tasks [0]",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Text::styled(" ", Style::default().fg(Color::Black).bg(Color::White)),
        Text::styled(
            "\u{E0B0}",
            Style::default().fg(Color::White).bg(Color::Black),
        ),
    ];
    let paragraph = Paragraph::new(text.iter());
    frame.render_widget(paragraph, area);
}
