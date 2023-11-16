use structs::app::AppState;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use ratatui::text::{Span, Line};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

const TEMPLATE: &'static str = r#"
<Paragraph>
    <Line>
        <Span styles='{"fg": "white", "bg": "black"}'> </Span>
        <Span styles='{"bg": "black"}'>{{value}}</Span>
        <Span styles='{"fg": "white", "bg": "black"}'> </Span>
        <Span styles='{"fg": "black", "bg": "white"}'>\u{E0B0}</Span>
        <Span styles='{"fg": "black", "bg": "white"}'>Running Tasks</Span>
        <Span styles='{"fg": "black", "bg": "white"}'> </Span>
        <Span styles='{"fg": "white", "bg": "black"}'>\u{E0B0}</Span>
    </Line>
</Paragraph>"#;


pub fn render(frame: &mut Frame, store: &AppState, area: Rect)
{
    let value = store.json_store["mode"]["symbol"]
        .as_str()
        .expect("JSON Error");
    let text = Line::from(vec![
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
