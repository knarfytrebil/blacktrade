use tui::Frame; 
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::widgets::Widget;

use structs::app::AppState;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect) 
where 
    B: Backend
{
    Paragraph::default()
        .text(&app.mode.symbol)
        .render(frame, &area);
}
