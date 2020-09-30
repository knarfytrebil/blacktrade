use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::Rect;

use tui::widgets::Paragraph;
use tui::text::Spans;
use tui::Frame;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect)
where
    B: Backend,
{
    let text = vec![Spans::from(app.command.clone())];
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, area);
}
