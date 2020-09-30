use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Block;

use tui::widgets::{Paragraph, Wrap};
use tui::text::{Spans, Span};
use tui::Frame;

use structs::app::AppState;

fn get_buffer(area_height: u16, lines: Vec<String>) -> Vec<String> {
    let buffer_start = match area_height as usize <= lines.len() {
        false => 0,
        true => lines.len() - area_height as usize
    };
    (&lines[buffer_start..]).to_vec()
}

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect)
where
    B: Backend,
{
    let buf = get_buffer(area.height, app.console_output_lines.clone());
    let text: Vec<Spans> = buf.iter()
        .map(|l| { 
            Spans::from(Span::raw(l)) 
        }).collect();
    let paragraph = Paragraph::new(text)
        .block(Block::default())
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
