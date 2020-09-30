use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Block;

use tui::widgets::{Paragraph, Wrap};
use tui::text::{Spans, Span};
use tui::Frame;

use structs::app::AppState;

fn get_buffer(area_height: u16, txt: String) -> String {
    let mut lines: Vec<usize> = txt.lines().map(|line| line.len()).collect();
    let line_count = lines.len();
    let drained: Vec<usize> = match (line_count as u16).checked_sub(area_height) {
        Some(x) if x >= 0 as u16 => lines.drain(x as usize..line_count).collect(),
        None | Some(_) => lines,
    };
    let drained_bytes: usize = drained.iter().sum();
    match txt.len() {
        l if l <= drained_bytes => txt,
        l if l > drained_bytes => txt[l - drained_bytes - drained.len()..l].to_string(),
        _ => txt,
    }
}

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect)
where
    B: Backend,
{
    let text = Spans::from(
        Span::raw(
            get_buffer(area.height, app.console_txt.clone())
        )
    );
    let paragraph = Paragraph::new(text)
        .block(Block::default())
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
