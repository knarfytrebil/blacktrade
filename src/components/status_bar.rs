use tui::Frame; 
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Paragraph, Text};
use tui::widgets::Widget;

use structs::app::AppState;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect) 
where 
    B: Backend
{
    let text = [Text::raw(&app.mode.symbol)];
    Paragraph::new(text.iter()).render(frame, area);
}
