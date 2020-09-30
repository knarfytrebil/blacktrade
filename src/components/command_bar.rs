use structs::app::AppState;
use treexml::Document;
use tui::backend::Backend;
use tui::layout::Rect;

use tui::widgets::Paragraph;
use tui::text::Spans;
use tui::Frame;

const DATA: &'static str = r#"
    <Paragraph>
        <Spans>{app.command}</Spans>
    </Paragraph>"#;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect)
where
    B: Backend,
{
    let paragraph = Paragraph::new(vec![Spans::from(
        app.command.clone()
    )]);
    frame.render_widget(paragraph, area);
}

// <Paragraph>
//     <Spans>{ app.command }</Spans>
// </Paragraph>
