use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::Rect;

use tui::widgets::{Paragraph, Text};
use tui::Frame;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect)
where
    B: Backend,
{
    let text = [Text::raw(app.mode.symbol.clone())];
    let paragraph = Paragraph::new(text.iter());
    frame.render_widget(paragraph, area);
}
