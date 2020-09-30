use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::text::Spans;
use tui::Frame;

pub fn render<B>(frame: &mut Frame<B>, store: &AppState, area: Rect)
where
    B: Backend,
{
    let paragraph = Paragraph::new(vec![Spans::from(
        store.command.clone()
    )]);
    frame.render_widget(paragraph, area);
}
